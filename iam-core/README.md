# IAM Core - 身份与访问管理核心系统

基于 Rust 技术栈的身份与访问管理（IAM）核心系统，采用领域驱动设计（DDD）、命令查询职责分离（CQRS）和事件溯源（Event Sourcing）架构模式。

## 技术栈

- **Web 框架**: Axum
- **数据库**: MySQL (通过 SQLx 和 SeaORM)
- **异步运行时**: Tokio
- **序列化**: Serde + Serde JSON
- **验证**: Validator
- **认证**: JWT + BCrypt
- **日志**: Tracing
- **错误处理**: ThisError + Anyhow

## 架构特点

- **DDD 分层架构**: 严格分离领域层、应用层、基础设施层和接口层
- **CQRS 模式**: 命令端和查询端分离
- **事件溯源**: 所有状态变更通过事件记录
- **多租户支持**: 设计支持多租户架构
- **类型安全**: Rust 的类型系统保证编译时安全

## 项目结构

```
src/
├── domain/                 # 领域层
│   └── identity_access/   # 身份访问管理限界上下文
│       ├── aggregates/    # 聚合根
│       ├── commands.rs    # 命令对象
│       ├── events.rs      # 领域事件
│       └── value_objects/ # 值对象
├── application/           # 应用层
│   ├── services/         # 应用服务
│   └── dtos.rs          # 数据传输对象
├── infrastructure/       # 基础设施层
│   └── persistence/     # 持久化
│       ├── event_store.rs    # 事件存储
│       └── projectors.rs     # 事件投影器
├── interface/           # 接口层
│   ├── handlers/        # 请求处理器
│   ├── middleware/      # 中间件
│   └── routes/          # 路由定义
├── config/             # 配置管理
└── error.rs           # 错误处理
```

## 快速开始

### 1. 环境准备

确保已安装以下软件：
- Rust 1.70+
- MySQL 8.0+
- Git

### 2. 克隆项目

```bash
git clone <repository-url>
cd iam-core
```

### 3. 配置环境变量

复制环境变量模板文件：
```bash
cp env.example .env
```

编辑 `.env` 文件，配置数据库连接和其他参数：
```env
DATABASE_URL=mysql://username:password@localhost:3306/iam_core
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
JWT_SECRET=your-super-secret-jwt-key-here
ENVIRONMENT=development
```

### 4. 创建数据库

在 MySQL 中创建数据库：
```sql
CREATE DATABASE iam_core;
```

### 5. 运行项目

```bash
# 安装依赖
cargo build

# 运行服务器
cargo run
```

服务器将在 `http://localhost:3000` 启动。

## API 接口

### 健康检查
```bash
GET /health
```

### 用户管理

#### 注册用户
```bash
POST /api/v1/users
Content-Type: application/json

{
  "username": "testuser",
  "email": "test@example.com",
  "password": "password123"
}
```

#### 获取用户信息
```bash
GET /api/v1/users/{user_id}
```

#### 获取用户列表
```bash
GET /api/v1/users
```

### 认证

#### 用户登录
```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "password123"
}
```

#### 刷新令牌
```bash
POST /api/v1/auth/refresh
```

#### 用户登出
```bash
POST /api/v1/auth/logout
```

## 数据库迁移

项目使用 SQLx 进行数据库迁移。迁移文件位于 `migrations/` 目录：

- `001_create_events_table.sql` - 创建事件存储表
- `002_create_users_view_table.sql` - 创建用户读模型表
- `003_create_tenants_table.sql` - 创建租户表

## 测试

运行单元测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test user_aggregate_tests
```

## 开发指南

### 添加新的聚合根

1. 在 `src/domain/identity_access/aggregates/` 中创建新的聚合根
2. 在 `src/domain/identity_access/events.rs` 中添加相关事件
3. 在 `src/domain/identity_access/commands.rs` 中添加相关命令
4. 在 `src/application/services/` 中创建应用服务
5. 在 `src/infrastructure/persistence/projectors.rs` 中添加事件投影器
6. 在 `src/interface/handlers/` 中添加 API 处理器

### 添加新的 API 端点

1. 在 `src/interface/handlers/` 中创建处理器函数
2. 在 `src/interface/routes/` 中添加路由定义
3. 更新 `src/interface/routes/mod.rs` 中的路由配置

## 部署

### Docker 部署

```bash
# 构建镜像
docker build -t iam-core .

# 运行容器
docker run -p 3000:3000 --env-file .env iam-core
```

### 生产环境配置

1. 设置 `ENVIRONMENT=production`
2. 配置强密码的 `JWT_SECRET`
3. 使用生产级数据库连接
4. 配置适当的 CORS 策略
5. 启用 HTTPS

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 联系方式

如有问题或建议，请通过以下方式联系：
- 创建 Issue
- 发送邮件至 [your-email@example.com]

## 更新日志

### v0.1.0 (2024-01-XX)
- 初始版本发布
- 实现基础用户管理功能
- 支持用户注册、更新、停用
- 实现事件溯源和 CQRS 架构
- 添加 Web API 接口
- 支持 JWT 认证（待完善）
