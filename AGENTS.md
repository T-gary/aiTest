# 极简商城 — 项目记忆

## 规则（必须遵守）

1. 代码产出目录：`/home/chengzhen/桌面/教学/project/1/pj`（服务器 192.168.1.1）。
   本地开发目录：`D:\OpenCode\pj`，同步后与服务器目录结构一致。
2. 每次改动都要更新 `README.md` 里的**版本号**和**功能清单**。
3. 本地端口固定 **5050**：Rust 后端同时托管前端构建产物，只暴露一个端口。
4. 目标平台：PC 浏览器 + 安卓浏览器（响应式，不打包 APK）。
5. 技术栈固定：Vue3 + Rust(axum) + MySQL，数据库名 `pj_shop`。

## 服务器

- 应用 + MySQL 同机：`192.168.1.1`，root / 123456（生产环境请改）。
- SSH 22 端口在本机不通，部署用人工 `scp`/`rsync`。

## 结构

```
pj/
  server/           Rust 后端（axum + sqlx），监听 5050
    src/main.rs     全部路由 + 建表 + 初始管理员
    Cargo.toml
  web/              Vue3 前端
    src/views/Shop.vue    前台：左侧导航 + 右侧商品
    src/views/Login.vue   登录 / 注册（注册需审核）
    src/views/Admin.vue   后台：看板 / 商品录入 / 账号审核
  deploy/           服务器部署脚本
  README.md
```

## 关键决策

- 单一角色 + JWT（token 存 localStorage，7 天有效）。
- 注册默认 `pending`，管理员审核为 `approved` 后才能登录。
- 商品字段：name / price / image_url（只存外链）/ description / created_by / created_at。
- 价格按「分」存（`price_cents`），避免浮点误差。
- 看板指标：商品总数、账号总数、累计登录次数、每个账号登录次数/最近登录/上架产品数、最近 50 条登录记录。
- 初始管理员：`admin / 123456`（首次启动自动创建，请尽快改密）。

## 本地开发环境（Windows）

- Rust：`D:\Value\Rust`（toolchain `stable-x86_64-pc-windows-gnu`），用 `D:\Value\cargo.cmd` 调用。
- MinGW：`D:\Value\mingw\mingw64\bin`。
- MySQL：`D:\Value\MySQL`（8.4.6，port 3306，root/123456，库 `pj_shop`）。

## 待办

- [ ] 部署脚本根据服务器实际可用的 SSH 通道补全。
- [ ] 管理员改密码接口（当前无）。
