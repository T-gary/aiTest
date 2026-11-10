# 极简商城

版本：**v0.1.1**

Vue3 + Rust(axum) + MySQL 的极简商城。前台只展示商品，没有买家和商家之分；后台提供账号注册/审核、商品录入和运营看板。PC 与安卓浏览器共用同一套响应式页面。

## 功能

### 前台 `/`

- 左侧导航栏 + 右侧商品栅格，窄屏（≤820px）导航自动收成抽屉。
- 商品卡片：图片、名称、描述、价格。
- 按商品名称搜索、分页（每页 12 条）。

### 后台 `/admin`

- **登录 / 注册**：`/login`，注册后状态为「待审核」，人工审核通过才能登录。
- **看板**：商品总数、账号总数、累计登录次数；每个账号的登录次数、最近登录时间、上架产品数；最近 50 条登录记录。
- **商品录入**：名称、价格、图片外链 URL、描述，提交后立即出现在前台。
- **商品列表**：分页查看，可删除自己录入的商品（管理员可删全部）。
- **账号审核**：查看全部账号状态，一键「审核通过」。

## 技术栈

| 层 | 选型 |
| --- | --- |
| 前端 | Vue 3 + Vue Router + Vite |
| 后端 | Rust + axum 0.8 + sqlx 0.8 |
| 数据库 | MySQL 8.4，库名 `pj_shop` |
| 鉴权 | JWT（Bearer，7 天有效）+ bcrypt 密码哈希 |

后端同时托管前端构建产物，**只需一个端口 5050**。

## 数据表

| 表 | 说明 |
| --- | --- |
| `users` | 账号，`status` 为 pending / approved / rejected，`is_admin` 标记管理员 |
| `products` | 商品，价格以「分」存 `price_cents`，`created_by` 关联录入人 |
| `login_logs` | 每次登录写一条记录，供看板统计 |

表结构在服务启动时自动创建，无需手动执行 SQL。

## 目录结构

```
pj/
  server/            Rust 后端
    src/main.rs      路由、建表、初始管理员
    Cargo.toml
  web/               Vue3 前端
    src/App.vue      左右分栏布局
    src/views/       Shop / Login / Admin
  deploy/            服务器部署脚本
  AGENTS.md          项目记忆（规则、环境、决策）
  README.md
```

## 本地运行

前置：Rust、Node 18+、MySQL 8，且已建库 `pj_shop`。

```bash
# 1. 前端构建
cd web && npm install && npm run build

# 2. 后端启动（默认连 mysql://root:123456@127.0.0.1:3306/pj_shop）
cd ../server && cargo run --release
```

打开 http://127.0.0.1:5050 。

自动化冒烟测试（需后端已启动；会创建一个随机测试账号并保留在库中，可自行删除）：

```bash
node smoke.mjs
```

可用环境变量覆盖配置：

| 变量 | 默认值 |
| --- | --- |
| `DATABASE_URL` | `mysql://root:123456@127.0.0.1:3306/pj_shop` |
| `JWT_SECRET` | `pj-shop-dev-secret`（生产必须改） |
| `PORT` | `5050` |
| `WEB_DIST` | `../web/dist` |

首次启动会创建管理员 `admin / 123456`，请登录后尽快修改。

## 部署

目标：服务器 `192.168.1.1`，目录 `/home/chengzhen/桌面/教学/project/1/pj`。

```bash
# 本机构建后上传（在 D:\OpenCode\pj 下）
scp -r web/dist server deploy root@192.168.1.1:/home/chengzhen/桌面/教学/project/1/pj/
```

服务器上安装 Rust 工具链后 `cargo build --release`，或直接上传交叉编译好的二进制。启动：

```bash
DATABASE_URL='mysql://root:123456@127.0.0.1:3306/pj_shop' \
JWT_SECRET='请替换为随机串' \
PORT=5050 \
/home/chengzhen/桌面/教学/project/1/pj/server/target/release/pj-shop
```

> 本机 192.168.1.1 的 22 端口当前不通，部署需走可用的 SSH 通道或人工拷贝。

## 版本历史

### v0.1.1

- 修复窄屏抽屉导航：遮罩改为独立层，菜单按钮文字不再换行/不可见。

### v0.1.0

- 初始版本：前后台同一 SPA，左右分栏商品页、搜索分页。
- 账号注册 + 人工审核 + JWT 登录。
- 商品录入/删除，图片用外链 URL。
- 运营看板：登录次数、最近登录、账号上架产品数。
