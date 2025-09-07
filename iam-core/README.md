# IAM Core - 身份与访问管理核心系统

基于 Rust 技术栈的身份与访问管理（IAM）核心系统，采用领域驱动设计（DDD）、命令查询职责分离（CQRS）和事件溯源（Event Sourcing）架构模式。

## 🚀 快速开始

```bash
# 克隆项目
git clone <repository-url>
cd iam-core

# 配置环境
cp env.example .env
# 编辑 .env 文件

# 启动数据库
docker-compose up -d mysql

# 运行应用
cargo run
```

## 📚 文档

完整的文档位于 [doc/](./doc/) 目录：

- **[快速入门](./doc/快速入门.md)** - 快速上手指南
- **[系统架构](./doc/系统架构.md)** - 详细的系统架构设计
- **[接口文档](./doc/接口文档.md)** - RESTful API 接口文档
- **[交互式 API 文档](./doc/api/)** - Swagger UI 文档
- **[部署指南](./doc/部署指南.md)** - 部署和运维指南
- **[测试指南](./doc/测试指南.md)** - 测试相关文档
- **[功能完善](./doc/功能完善.md)** - 系统完善过程和功能总结
- **[完整设计](./doc/完整设计.md)** - 系统详细设计文档
- **[设计补充](./doc/设计补充.md)** - 设计补充说明
- **[开发计划](./doc/开发计划.md)** - 开发任务与规划

## 🏗️ 技术栈

- **后端**: Rust + Axum + MySQL + SQLx + SeaORM
- **认证**: JWT + BCrypt
- **部署**: Docker + Docker Compose + Nginx
- **监控**: 自定义指标收集 + 健康检查

## ✨ 核心特性

- **类型安全**: Rust 的类型系统保证编译时安全
- **高性能**: 异步处理和零成本抽象
- **可扩展**: 事件驱动和微服务架构
- **可维护**: 清晰的分层和模块化设计
- **可观测**: 完整的监控和日志系统
- **可部署**: Docker 容器化支持

## 🎯 架构模式

- **DDD (领域驱动设计)**: 清晰的领域模型和业务逻辑
- **CQRS (命令查询职责分离)**: 读写分离的架构
- **Event Sourcing (事件溯源)**: 完整的事件历史记录
- **多租户支持**: 租户隔离和权限管理

## 🔧 开发

```bash
# 运行测试
cargo test

# 运行集成测试
cargo test --test integration_tests

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 生成 API 文档
make docs
```

## 🐳 部署

```bash
# 使用 Docker Compose
docker-compose up -d

# 或构建 Docker 镜像
docker build -t iam-core .
docker run -p 3000:3000 --env-file .env iam-core
```

## 📊 状态

- ✅ **Web框架集成** - 完整的Axum Web API
- ✅ **认证授权系统** - JWT认证和中间件
- ✅ **数据库迁移** - 完整的数据库结构
- ✅ **领域模型扩展** - 用户和角色管理
- ✅ **查询服务** - 读模型和查询优化
- ✅ **错误处理** - 完善的错误管理
- ✅ **配置管理** - 环境变量和配置
- ✅ **中间件系统** - 认证、验证、日志
- ✅ **监控系统** - 指标收集和健康检查
- ✅ **Docker支持** - 完整的容器化部署
- ✅ **测试覆盖** - 单元测试和集成测试框架

## 🤝 贡献

欢迎贡献！请查看 [贡献指南](./CONTRIBUTING.md) 了解如何参与项目开发。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

**需要帮助？** 查看 [文档目录](./doc/README.md) 或提交 Issue。
