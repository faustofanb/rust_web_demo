# Rust Web Demo - Spring Boot 迁移验证项目

这是一个基于 Rust 的 Web 应用，用于验证从 Spring Boot 迁移到 Rust 的可行性。

## 技术栈

- **Web框架**: Axum
- **数据库**: MySQL + SQLx
- **HTTP客户端**: Reqwest
- **认证**: OAuth2 + JWT
- **缓存**: Redis
- **消息队列**: Kafka
- **配置管理**: Dotenv
- **监控**: Prometheus

## 项目结构

```
src/
├── main.rs                 # 应用入口
├── config/                 # 配置管理
│   ├── mod.rs
│   └── app_config.rs
├── errors/                 # 错误处理
│   └── mod.rs
├── handlers/               # API处理器
│   ├── mod.rs
│   ├── auth_handlers.rs
│   ├── user_handlers.rs
│   └── health_handlers.rs
├── middleware/             # 中间件
│   ├── mod.rs
│   ├── cors.rs
│   └── logging.rs
├── models/                 # 数据模型
│   ├── mod.rs
│   └── user.rs
├── repositories/           # 数据访问层
│   ├── mod.rs
│   └── user_repository.rs
├── services/               # 业务逻辑层
│   ├── mod.rs
│   ├── auth_service.rs
│   └── user_service.rs
└── utils/                  # 工具函数
    ├── mod.rs
    └── response.rs
migrations/                 # 数据库迁移
└── 001_create_users_table.sql
```

## 快速开始

### 1. 环境准备

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 MySQL
# macOS
brew install mysql
brew services start mysql

# 创建数据库
mysql -u root -p
CREATE DATABASE rust_web_demo;
```

### 2. 配置环境变量

```bash
# 复制环境变量模板
cp env.example .env

# 编辑 .env 文件，配置数据库连接等信息
```

### 3. 运行应用

```bash
# 安装依赖
cargo build

# 运行数据库迁移
sqlx migrate run

# 启动应用
cargo run
```

### 4. 测试API

```bash
# 健康检查
curl http://localhost:3000/health

# 用户注册
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'

# 用户登录
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'

# 获取用户列表
curl http://localhost:3000/api/users
```

## API 端点

### 健康检查
- `GET /health` - 基础健康检查
- `GET /ready` - 就绪检查（包含数据库连接检查）

### 认证
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/me` - 获取当前用户信息

### 用户管理
- `GET /api/users` - 获取用户列表
- `POST /api/users` - 创建用户
- `GET /api/users/:id` - 获取指定用户
- `PUT /api/users/:id` - 更新用户
- `DELETE /api/users/:id` - 删除用户

## 开发命令

```bash
# 开发模式运行
cargo run

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 构建发布版本
cargo build --release
```

## 性能对比

与 Spring Boot 相比的预期性能提升：

- **吞吐量**: 3-5倍提升
- **延迟**: 50-70%降低
- **内存使用**: 60-80%减少
- **启动时间**: 5-10倍加快

## 迁移验证要点

1. **功能完整性**: 验证所有 Spring Boot 功能是否能在 Rust 中实现
2. **性能表现**: 对比实际性能数据
3. **开发效率**: 评估开发体验和效率
4. **维护成本**: 分析长期维护的复杂度
5. **团队适应性**: 评估团队学习成本

## 下一步计划

- [ ] 添加 Redis 缓存集成
- [ ] 实现 Kafka 消息队列
- [ ] 集成 Prometheus 监控
- [ ] 添加单元测试和集成测试
- [ ] 实现 JWT 中间件
- [ ] 添加 API 文档生成
- [ ] 容器化部署配置
