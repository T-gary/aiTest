use std::net::SocketAddr;

use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct App {
    db: MySqlPool,
    secret: String,
}

type Ok<T> = Result<T, ApiError>;

struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

fn bad(msg: &str) -> ApiError {
    ApiError(StatusCode::BAD_REQUEST, msg.to_string())
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    uid: i64,
    adm: bool,
    exp: usize,
}

fn token(a: &App, uid: i64, adm: bool) -> String {
    let exp = chrono::Utc::now().timestamp() as usize + 7 * 24 * 3600;
    encode(
        &Header::default(),
        &Claims { uid, adm, exp },
        &EncodingKey::from_secret(a.secret.as_bytes()),
    )
    .unwrap()
}

fn claims(a: &App, headers: &HeaderMap) -> Ok<Claims> {
    let raw = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError(StatusCode::UNAUTHORIZED, "未登录".into()))?;
    decode::<Claims>(
        raw,
        &DecodingKey::from_secret(a.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|_| ApiError(StatusCode::UNAUTHORIZED, "登录已过期".into()))
}

fn admin(a: &App, headers: &HeaderMap) -> Ok<Claims> {
    let c = claims(a, headers)?;
    if !c.adm {
        return Err(ApiError(StatusCode::FORBIDDEN, "需要管理员".into()));
    }
    Ok(c)
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResp {
    token: String,
    username: String,
    is_admin: bool,
}

fn check_credentials(c: &Credentials) -> Ok<()> {
    if c.username.trim().len() < 2 || c.username.len() > 50 {
        return Err(bad("用户名需 2-50 个字符"));
    }
    if c.password.len() < 6 {
        return Err(bad("密码至少 6 位"));
    }
    Ok(())
}

async fn register(State(a): State<App>, Json(c): Json<Credentials>) -> Ok<Json<serde_json::Value>> {
    check_credentials(&c)?;
    let hash = bcrypt::hash(&c.password, 10).map_err(|e| bad(&e.to_string()))?;
    let r = sqlx::query("INSERT INTO users (username, password_hash, status, is_admin) VALUES (?, ?, 'pending', 0)")
        .bind(c.username.trim())
        .bind(hash)
        .execute(&a.db)
        .await;
    match r {
        Ok(_) => Ok(Json(json!({ "ok": true, "message": "注册成功，等待管理员审核" }))),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Err(bad("用户名已存在")),
        Err(e) => Err(e.into()),
    }
}

async fn login(State(a): State<App>, Json(c): Json<Credentials>) -> Ok<Json<LoginResp>> {
    let row: Option<(i64, String, String, i8)> =
        sqlx::query_as("SELECT id, password_hash, status, is_admin FROM users WHERE username = ?")
            .bind(c.username.trim())
            .fetch_optional(&a.db)
            .await?;
    let (id, hash, status, is_admin) = row.ok_or(bad("用户名或密码错误"))?;
    if !bcrypt::verify(&c.password, &hash).unwrap_or(false) {
        return Err(bad("用户名或密码错误"));
    }
    if status != "approved" {
        return Err(ApiError(StatusCode::FORBIDDEN, "账号未通过审核".into()));
    }
    sqlx::query("INSERT INTO login_logs (user_id) VALUES (?)")
        .bind(id)
        .execute(&a.db)
        .await?;
    Ok(Json(LoginResp {
        token: token(&a, id, is_admin != 0),
        username: c.username.trim().to_string(),
        is_admin: is_admin != 0,
    }))
}

#[derive(Serialize)]
struct Product {
    id: i64,
    name: String,
    price: f64,
    image_url: String,
    description: String,
    created_by: String,
    created_at: String,
}

#[derive(Deserialize)]
struct Page {
    page: Option<u32>,
    q: Option<String>,
}

const PRODUCT_SELECT: &str = "SELECT p.id, p.name, p.price_cents, p.image_url, p.description, u.username, DATE_FORMAT(p.created_at, '%Y-%m-%d %H:%i')
     FROM products p LEFT JOIN users u ON u.id = p.created_by";

fn to_product(row: (i64, String, i64, String, String, Option<String>, String)) -> Product {
    Product {
        id: row.0,
        name: row.1,
        price: row.2 as f64 / 100.0,
        image_url: row.3,
        description: row.4,
        created_by: row.5.unwrap_or_default(),
        created_at: row.6,
    }
}

async fn list_products(State(a): State<App>, Query(p): Query<Page>) -> Ok<Json<serde_json::Value>> {
    let page = p.page.unwrap_or(1).max(1);
    let size = 12u32;
    let q = p.q.unwrap_or_default();
    let like = format!("%{}%", q.trim());
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products p WHERE ? = '' OR p.name LIKE ?",
    )
    .bind(q.trim())
    .bind(&like)
    .fetch_one(&a.db)
    .await?;
    let sql = format!("{PRODUCT_SELECT} WHERE ? = '' OR p.name LIKE ? ORDER BY p.id DESC LIMIT ? OFFSET ?");
    let rows: Vec<(i64, String, i64, String, String, Option<String>, String)> = sqlx::query_as(&sql)
        .bind(q.trim())
        .bind(&like)
        .bind(size)
        .bind((page - 1) * size)
        .fetch_all(&a.db)
        .await?;
    Ok(Json(json!({
        "total": total,
        "page": page,
        "size": size,
        "items": rows.into_iter().map(to_product).collect::<Vec<_>>(),
    })))
}

#[derive(Deserialize)]
struct ProductInput {
    name: String,
    price: f64,
    image_url: Option<String>,
    description: Option<String>,
}

fn check_product(p: &ProductInput) -> Ok<i64> {
    let name = p.name.trim();
    if name.is_empty() || name.len() > 100 {
        return Err(bad("商品名称需 1-100 个字符"));
    }
    if !p.price.is_finite() || p.price < 0.0 {
        return Err(bad("价格不合法"));
    }
    Ok((p.price * 100.0).round() as i64)
}

async fn create_product(
    State(a): State<App>,
    headers: HeaderMap,
    Json(p): Json<ProductInput>,
) -> Ok<Json<serde_json::Value>> {
    let c = claims(&a, &headers)?;
    let cents = check_product(&p)?;
    let r = sqlx::query(
        "INSERT INTO products (name, price_cents, image_url, description, created_by) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(p.name.trim())
    .bind(cents)
    .bind(p.image_url.unwrap_or_default())
    .bind(p.description.unwrap_or_default())
    .bind(c.uid)
    .execute(&a.db)
    .await?;
    Ok(Json(json!({ "ok": true, "id": r.last_insert_id() })))
}

async fn delete_product(
    State(a): State<App>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Ok<Json<serde_json::Value>> {
    let c = claims(&a, &headers)?;
    let owner: Option<i64> = sqlx::query_scalar("SELECT created_by FROM products WHERE id = ?")
        .bind(id)
        .fetch_optional(&a.db)
        .await?;
    let owner = owner.ok_or(ApiError(StatusCode::NOT_FOUND, "商品不存在".into()))?;
    if !c.adm && owner != c.uid {
        return Err(ApiError(StatusCode::FORBIDDEN, "只能删除自己录入的商品".into()));
    }
    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&a.db)
        .await?;
    Ok(Json(json!({ "ok": true })))
}

#[derive(Serialize)]
struct Account {
    id: i64,
    username: String,
    status: String,
    is_admin: bool,
    created_at: String,
    login_count: i64,
    last_login: Option<String>,
    product_count: i64,
}

async fn stats(State(a): State<App>, headers: HeaderMap) -> Ok<Json<serde_json::Value>> {
    admin(&a, &headers)?;
    let rows: Vec<(i64, String, String, i8, String, i64, Option<String>, i64)> = sqlx::query_as(
        "SELECT u.id, u.username, u.status, u.is_admin,
                DATE_FORMAT(u.created_at, '%Y-%m-%d %H:%i'),
                (SELECT COUNT(*) FROM login_logs l WHERE l.user_id = u.id),
                (SELECT DATE_FORMAT(MAX(l.created_at), '%Y-%m-%d %H:%i') FROM login_logs l WHERE l.user_id = u.id),
                (SELECT COUNT(*) FROM products p WHERE p.created_by = u.id)
         FROM users u ORDER BY u.id",
    )
    .fetch_all(&a.db)
    .await?;
    let logins: Vec<(String, String)> = sqlx::query_as(
        "SELECT u.username, DATE_FORMAT(l.created_at, '%Y-%m-%d %H:%i')
         FROM login_logs l JOIN users u ON u.id = l.user_id ORDER BY l.id DESC LIMIT 50",
    )
    .fetch_all(&a.db)
    .await?;
    let total_products: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(&a.db)
        .await?;
    let accounts: Vec<Account> = rows
        .into_iter()
        .map(|r| Account {
            id: r.0,
            username: r.1,
            status: r.2,
            is_admin: r.3 != 0,
            created_at: r.4,
            login_count: r.5,
            last_login: r.6,
            product_count: r.7,
        })
        .collect();
    Ok(Json(json!({
        "total_products": total_products,
        "total_accounts": accounts.len(),
        "total_logins": accounts.iter().map(|x| x.login_count).sum::<i64>(),
        "accounts": accounts,
        "recent_logins": logins.iter().map(| (u, t)| json!({"username": u, "at": t})).collect::<Vec<_>>(),
    })))
}

async fn approve_account(
    State(a): State<App>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Ok<Json<serde_json::Value>> {
    admin(&a, &headers)?;
    sqlx::query("UPDATE users SET status = 'approved' WHERE id = ?")
        .bind(id)
        .execute(&a.db)
        .await?;
    Ok(Json(json!({ "ok": true })))
}

async fn me(State(a): State<App>, headers: HeaderMap) -> Ok<Json<Claims>> {
    claims(&a, &headers).map(Json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:123456@127.0.0.1:3306/pj_shop".into());
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "pj-shop-dev-secret".into());
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(5050);

    let db = MySqlPoolOptions::new().max_connections(10).connect(&db_url).await?;
    migrate(&db).await?;
    bootstrap(&db).await?;

    let app = App { db, secret };
    let api = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me))
        .route("/products", get(list_products).post(create_product))
        .route("/products/{id}", delete(delete_product))
        .route("/admin/stats", get(stats))
        .route("/admin/accounts/{id}/approve", post(approve_account))
        .with_state(app);

    let static_dir = std::env::var("WEB_DIST").unwrap_or_else(|_| "../web/dist".into());
    let index = format!("{static_dir}/index.html");
    let router = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new(&static_dir).fallback(ServeFile::new(index)));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("pj-shop listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn migrate(db: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            username VARCHAR(50) NOT NULL UNIQUE,
            password_hash VARCHAR(100) NOT NULL,
            status ENUM('pending','approved','rejected') NOT NULL DEFAULT 'pending',
            is_admin TINYINT NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
    )
    .execute(db)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(100) NOT NULL,
            price_cents BIGINT NOT NULL DEFAULT 0,
            image_url VARCHAR(500) NOT NULL DEFAULT '',
            description TEXT NOT NULL,
            created_by BIGINT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            INDEX idx_created_by (created_by),
            CONSTRAINT fk_products_user FOREIGN KEY (created_by) REFERENCES users(id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
    )
    .execute(db)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS login_logs (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            user_id BIGINT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            INDEX idx_user (user_id),
            CONSTRAINT fk_logs_user FOREIGN KEY (user_id) REFERENCES users(id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn bootstrap(db: &MySqlPool) -> Result<(), sqlx::Error> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(db).await?;
    if count == 0 {
        let hash = bcrypt::hash("123456", 10).unwrap_or_default();
        sqlx::query("INSERT INTO users (username, password_hash, status, is_admin) VALUES ('admin', ?, 'approved', 1)")
            .bind(hash)
            .execute(db)
            .await?;
        println!("已创建初始管理员 admin / 123456，请尽早修改密码");
    }
    Ok(())
}
