# Rust CI

一个基于 Rust 构建的轻量级、自托管 CI（持续集成）系统。无需 Docker，无需复杂的部署流程，单一二进制文件即可运行。

English | [中文](#中文)

---

## Features

- 🚀 **Webhook Trigger** — 每个项目分配唯一的 Webhook URL，通过 HTTP 请求即可触发构建
- 📋 **Project Management** — 可视化管理多个项目，支持添加、编辑操作
- 🏗️ **Build Logs** — 每次构建自动记录 Shell 脚本执行日志，支持实时查看（`tail -f`）
- 📂 **Runtime Log Browser** — 内置运行日志浏览器，可浏览项目运行日志目录下的所有日志文件，并实时追踪（`tail -f -n 5000`）
- 🌐 **Bilingual UI** — 支持中文 / English 一键切换，默认中文
- 🔐 **User Auth** — 登录鉴权、用户管理、密码修改，内置管理员账号
- 💾 **SQLite** — 数据存储基于 SQLite，零运维成本

---

## Tech Stack

| 组件 | 技术选型 |
|------|---------|
| Web 框架 | [Axum](https://github.com/tokio-rs/axum) |
| 数据库 | [SQLite](https://www.sqlite.org/) via [sqlx](https://github.com/launchbadge/sqlx) |
| 模板引擎 | [Askama](https://github.com/djc/askama) |
| 前端样式 | [Tailwind CSS](https://tailwindcss.com/) (CDN) |
| 密码哈希 | [Argon2](https://docs.rs/argon2/) |
| 会话管理 | [tower-sessions](https://github.com/maxcountryman/tower-sessions) |
| 异步运行时 | [Tokio](https://tokio.rs/) |

---

## Quick Start

### Prerequisites

- Rust 1.75+（推荐使用 [rustup](https://rustup.rs/) 安装）

### Run

```bash
git clone https://github.com/asdtiang/rust-ci.git
cd rust-ci
cargo run
```

服务默认监听 `http://localhost:3000`。

### Default Admin

| 用户名 | 密码 |
|--------|------|
| `admin` | `admin456` |

---

## Usage

### 1. 添加项目

进入 **项目管理** 页，填写以下信息：

| 字段 | 说明 |
|------|------|
| **项目名称** | 项目的显示名称 |
| **Shell 脚本路径** | 构建时执行的脚本的绝对路径，例如 `/opt/deploy/myapp.sh` |
| **运行日志目录** | 项目运行时产生的日志所在的目录，例如 `/var/log/myapp`（注意：是**目录**，不是具体文件） |

项目创建后，系统会自动生成一个 **Webhook URL**。

### 2. 触发构建

向 Webhook URL 发起 HTTP 请求即可触发构建：

```bash
curl -X POST http://localhost:3000/webhook/<your-webhook-key>
```

构建过程中：
- Shell 脚本会被**异步执行**，不会阻塞 HTTP 响应
- 执行日志自动保存到项目根目录下的 `ci_log/` 文件夹
- 可在 **控制台** 页实时查看最近 50 次构建的状态

### 3. 查看构建日志

在控制台点击任意一条构建记录即可进入日志查看页，支持**实时刷新**（1 秒轮询）。

### 4. 浏览运行日志

在项目列表点击 **运行日志** 按钮，即可列出 `运行日志目录` 下所有文件，点击任意文件进入实时 `tail -f` 查看模式（自动加载最新 5000 行）。

---

## Project Structure

```
rust-ci/
├── src/
│   ├── main.rs               # 入口、路由注册、数据库初始化
│   ├── models.rs             # 数据库模型 (Project, Build, User)
│   ├── templates.rs          # Askama 模板结构体
│   ├── i18n.rs               # 轻量级国际化 (中/英)
│   └── handlers/
│       ├── mod.rs            # 公共工具函数 (get_lang, find_last_lines_offset)
│       ├── auth.rs           # 登录/登出/用户管理/语言切换
│       ├── projects.rs       # 项目 CRUD
│       ├── project_logs.rs   # 运行日志浏览 & tail-f API
│       ├── builds.rs         # 构建历史 & 构建日志 API
│       └── webhook.rs        # Webhook 触发构建
├── templates/
│   ├── base.html             # 基础布局 (Tailwind CSS + 导航栏)
│   ├── login.html
│   ├── dashboard.html        # 构建历史
│   ├── projects.html         # 项目列表 & 添加表单
│   ├── project_edit.html     # 项目编辑
│   ├── project_logs.html     # 运行日志文件列表
│   ├── project_log_viewer.html # 运行日志 tail-f 查看
│   ├── build_detail.html     # 构建日志 tail-f 查看
│   └── users.html            # 用户管理
├── ci_log/                   # 构建脚本执行日志（自动创建）
├── rust_ci.db                # SQLite 数据库（自动创建）
└── Cargo.toml
```

---

## API Reference

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/webhook/:key` | 触发构建 |
| `GET`  | `/api/builds/:id/logs?offset=N` | 获取构建日志增量 |
| `GET`  | `/api/projects/:id/logs/:filename?offset=N` | 获取运行日志增量 |

---

## Security Notes

- 当前 Session 配置为 `with_secure(false)`，适用于本地 HTTP 部署。若部署到公网，请启用 HTTPS 并将其改为 `true`。
- 运行日志 API 已做路径遍历防护，仅允许读取 `log_dir` 下一层的文件。

---

## License

MIT

---

<a name="中文"></a>

*以上内容即为中文版说明，English version coming soon.*
