# 极简商城

版本：**v0.2.2**

Vue3 + Rust(axum) + MySQL 的极简商城。前台只展示商品，没有买家和商家之分；后台提供账号注册/审核、商品录入和运营看板。PC 与安卓浏览器共用同一套响应式页面。

## 功能

### 前台 `/`

参考京东的三栏布局：

- **顶部**：搜索框（按商品名称搜索）。
- **左栏**：商品分类列表，带各分类商品数，点选即筛选（分类由商品自动汇总）。
- **中栏**：顶部促销轮播（后台勾选「首页轮播」的商品，4 秒自动切换、可点圆点切换）；下面是纵向商品列表，可向下滚动，每页 10 条。
- **右栏**：悬浮工具栏（购物车 / 我的 / 客服 / 桌面版 / 插件版 / 反馈 / 调研，仿京东）。
  - 购物车带红色件数角标，点进结算页；其余为占位灰显。
- **中间商品区**：PC 端栅格布局（每列 200px 起，自适应 3~6 列），中间区独立滚动，左右两栏固定，整页不出滚动条。
- **结算页 `/checkout`**：购物车明细表格（改数量、删除、小计）+ 粘性合计条（清空 / 去结算禁用）。
  - **纯前端**：存 `localStorage`，不落库、不下单支付。
- 断点：≤1100px 三栏收窄；≤760px 单列，分类横向滚动，工具栏贴右侧悬浮。

### 后台 `/admin`

- **登录 / 注册**：`/login`，注册后状态为「待审核」，人工审核通过才能登录。
- **看板**：商品总数、账号总数、累计登录次数；每个账号的登录次数、最近登录时间、上架产品数；最近 50 条登录记录。
- **商品录入**：名称、价格、分类、图片外链 URL、描述、是否首页轮播，提交后立即出现在前台。
- **商品列表**：分页查看，可删除自己录入的商品（管理员可删全部），管理员可一键切换「上轮播」。
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
| `products` | 商品，价格以「分」存 `price_cents`，`category` 分类，`featured` 是否进首页轮播，`created_by` 关联录入人 |
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

代码托管在 GitHub：`git@github.com:T-gary/aiTest.git`（分支 `main`）。服务器上拉取：

```bash
git clone git@github.com:T-gary/aiTest.git /home/chengzhen/桌面/教学/project/1/pj
```

本机改完推送：

```bash
git add -A && git commit -m "feat: ..." && git push
```

服务器首次构建：

```bash
cd /home/chengzhen/桌面/教学/project/1/pj

# 前端
cd web && npm ci && npm run build && cd ..

# 后端
cd server && cargo build --release && cd ..
```

启动：

```bash
DATABASE_URL='mysql://root:123456@127.0.0.1:3306/pj_shop' \
JWT_SECRET='请替换为随机串' \
PORT=5050 \
WEB_DIST=/home/chengzhen/桌面/教学/project/1/pj/web/dist \
/home/chengzhen/桌面/教学/project/1/pj/server/target/release/pj-shop
```

> 本机到 192.168.1.1 的 22 端口不通，因此用 Git 作为交付通道；`deploy/deploy.sh` 里的 scp 版本仅在 SSH 恢复后可用。

## 版本历史

### v0.2.2

- PC 端适配：右侧工具栏改为垂直结构固定在右边（不再折到列表下方）。
- 商品展示区改为响应式栅格（`auto-fill minmax(200px, 1fr)`），中间区独立滚动，左右两栏固定，整页不出现滚动条。
- 商品卡改为「图 + 卡身」结构，图片 4:3 定比，描述两行截断。
- PC 每页商品 20 条（原 10 条），更贴合栅格。

### v0.2.1

- 右侧购物车侧栏改为京东式悬浮工具栏（购物车/我的/客服/桌面版/插件版/反馈/调研），购物车带件数红色角标。
- 购物车内容迁移到独立结算页 `/checkout`：明细表格、数量增减、删除、清空、粘性合计条，「去结算」禁用（演示项目）。
- 手机端工具栏贴右侧悬浮，内容区留出安全间距。

### v0.2.0

- 首页改版为京东式三栏：左分类、中轮播 + 纵向商品列表、右购物车侧栏，顶部搜索框。
- 商品新增 `category`（分类，左栏自动汇总）与 `featured`（首页轮播）字段，旧库自动补列。
- 新增接口：`GET /api/categories`、`GET /api/products?category=&featured=&size=`、`POST /api/admin/products/{id}/featured`。
- 购物车为纯前端实现（localStorage），可加减/删除/清空/看合计。
- 后台商品录入支持分类与「首页轮播」，商品列表可切换轮播状态。
- 顶部导航改为横向条，去掉旧的抽屉侧栏。

### v0.1.1

- 修复窄屏抽屉导航：遮罩改为独立层，菜单按钮文字不再换行/不可见。

### v0.1.0

- 初始版本：前后台同一 SPA，左右分栏商品页、搜索分页。
- 账号注册 + 人工审核 + JWT 登录。
- 商品录入/删除，图片用外链 URL。
- 运营看板：登录次数、最近登录、账号上架产品数。
