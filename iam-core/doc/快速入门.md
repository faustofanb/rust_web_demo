# 快速开始指南

本指南将帮助您快速了解和使用 IAM Core 系统。

## 📋 系统要求

### 必需软件
- **Rust**: 1.70+ 
- **MySQL**: 8.0+
- **Docker**: 20.0+ (可选，用于容器化部署)

### 推荐软件
- **Git**: 用于版本控制
- **VS Code**: 推荐的代码编辑器
- **MySQL Workbench**: 数据库管理工具

## 🚀 快速安装

### 1. 克隆项目
```bash
git clone <repository-url>
cd iam-core
```

### 2. 配置环境
```bash
# 复制环境变量模板
cp env.example .env

# 编辑配置文件
vim .env
```

### 3. 启动数据库
```bash
# 使用 Docker Compose (推荐)
docker-compose up -d mysql

# 或手动启动 MySQL 服务
```

### 4. 运行应用
```bash
# 安装依赖
cargo build

# 启动服务
cargo run
```

## 🔧 基本配置

### 环境变量配置
```env
# 数据库配置
DATABASE_URL=mysql://username:password@localhost:3306/iam_core
DATABASE_MAX_CONNECTIONS=10
DATABASE_MIN_CONNECTIONS=1

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
CORS_ORIGINS=http://localhost:3000,http://localhost:3001

# JWT 配置
JWT_SECRET=your-super-secret-jwt-key-here
JWT_EXPIRATION_HOURS=24

# 环境
ENVIRONMENT=development
```

## 📖 基本概念

### 核心概念
- **租户 (Tenant)**: 多租户系统中的租户隔离
- **用户 (User)**: 系统中的用户账户
- **角色 (Role)**: 用户权限的角色定义
- **权限 (Permission)**: 具体的操作权限
- **事件 (Event)**: 系统中发生的事件记录

### 架构模式
- **DDD**: 领域驱动设计
- **CQRS**: 命令查询职责分离
- **Event Sourcing**: 事件溯源

## 🎯 第一个请求

### 1. 健康检查
```bash
curl http://localhost:3000/health
```

### 2. 注册用户
```bash
curl -X POST http://localhost:3000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

### 3. 用户登录
```bash
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

## 🔍 验证安装

### 检查服务状态
```bash
# 检查健康状态
curl http://localhost:3000/health

# 检查数据库连接
curl http://localhost:3000/metrics
```

### 查看日志
```bash
# 查看应用日志
cargo run 2>&1 | tee app.log

# 或使用 Docker
docker-compose logs -f iam-core
```

## 🐛 常见问题

### 数据库连接失败
```bash
# 检查 MySQL 服务状态
docker-compose ps mysql

# 检查数据库连接
mysql -h localhost -u root -p
```

### 端口冲突
```bash
# 检查端口占用
lsof -i :3000

# 修改端口配置
vim .env
# 修改 SERVER_PORT=3001
```

### 权限问题
```bash
# 检查文件权限
ls -la

# 修复权限
chmod +x target/debug/iam-core
```

## 📚 下一步

1. **学习架构**: 阅读 [系统架构文档](./architecture.md)
2. **API 开发**: 查看 [API 文档](./api.md)
3. **部署生产**: 参考 [部署指南](./deployment.md)
4. **测试系统**: 查看 [测试指南](./testing.md)

## 💡 提示

- 开发环境建议使用 `ENVIRONMENT=development`
- 生产环境必须设置强密码和安全的 JWT Secret
- 定期备份数据库数据
- 监控系统日志和指标

---

**需要帮助？** 查看 [API 文档](./api.md) 或提交 Issue。
