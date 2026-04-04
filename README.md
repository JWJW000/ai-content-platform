# AI 内容生产 + 自动发布平台

基于 Rust 的 AI 内容自动生成和发布平台，支持小红书、微信公众号等平台。

## 🚀 快速部署

### 方式一：Docker 一键部署（推荐）

```bash
# 克隆代码
git clone https://github.com/YOUR_USERNAME/ai-content-platform.git
cd ai-content-platform

# 配置环境变量
cp .env.production .env
# 编辑 .env 填入配置

# 启动服务
docker-compose -f docker-compose.prod.yml up -d
```

访问 http://auto.5wjw.cn 即可使用。

### 方式二：本地开发

```bash
# 后端
cd ai-content-platform
cargo run

# 前端
cd frontend
npm install
npm run dev
```

## 📋 功能

- ✅ 任务管理（创建、启动、停止、删除）
- ✅ AI 内容生成（支持 OpenAI API）
- ✅ 内容审核工作流（生成 → 审核 → 发布）
- ✅ 小红书自动发布框架
- ✅ Web 管理界面
- ✅ 日志系统

## 🔧 配置

### 环境变量 (.env)

```bash
# 数据库密码
DB_PASSWORD=your_secure_password

# OpenAI API Key（可选，不填使用模拟数据）
OPENAI_API_KEY=sk-xxx
```

### 域名配置

已配置域名 `auto.5wjw.cn`，Nginx 配置见 `nginx-prod.conf`。

## 🐳 Docker 部署

```bash
# 构建并启动
docker-compose -f docker-compose.prod.yml up -d --build

# 查看日志
docker-compose -f docker-compose.prod.yml logs -f

# 停止服务
docker-compose -f docker-compose.prod.yml down
```

## 📁 项目结构

```
ai-content-platform/
├── src/                    # Rust 后端源码
│   ├── main.rs
│   ├── handler/            # HTTP 处理器
│   ├── service/            # 业务逻辑
│   ├── repository/         # 数据访问
│   ├── scheduler/          # 任务调度
│   ├── publisher/          # 发布系统
│   └── models/            # 数据模型
├── frontend/               # React 前端
│   ├── src/
│   │   ├── components/     # UI 组件
│   │   └── lib/          # API 调用
│   └── dist/             # 构建产物
├── docker-compose.yml      # 开发环境
├── docker-compose.prod.yml # 生产环境
├── Dockerfile             # 后端构建
└── nginx-prod.conf        # Nginx 配置
```

## 🔌 API 接口

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/tasks | 获取任务列表 |
| POST | /api/tasks | 创建任务 |
| POST | /api/tasks/:id/start | 启动任务 |
| POST | /api/tasks/:id/stop | 停止任务 |
| DELETE | /api/tasks/:id | 删除任务 |
| GET | /api/contents | 获取内容列表 |
| POST | /api/contents/:id/review | 审核内容 |
| GET | /api/accounts | 获取账号列表 |
| POST | /api/accounts | 添加账号 |
| GET | /api/logs | 获取日志 |

## 📝 工作流程

```
1. 创建任务（设置平台、prompt、cron 表达式）
2. 启动任务（调度器按 cron 执行）
3. AI 生成内容（状态：pending_review）
4. 人工审核（通过/拒绝）
5. 审核通过后自动发布（状态：published）
```

## 🔒 安全注意

- 首次部署请修改 `DB_PASSWORD`
- 生产环境建议启用 HTTPS
- 定期备份数据库

## License

MIT
