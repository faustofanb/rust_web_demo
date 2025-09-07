.PHONY: help build test run clean docs swagger-ui

# 默认目标
help:
	@echo "IAM Core 项目构建工具"
	@echo ""
	@echo "可用命令:"
	@echo "  build        - 构建项目"
	@echo "  test         - 运行测试"
	@echo "  run          - 运行服务器"
	@echo "  clean        - 清理构建文件"
	@echo "  docs         - 生成 OpenAPI 文档"
	@echo "  swagger-ui   - 启动 Swagger UI 服务器"
	@echo "  docker       - 使用 Docker 运行"
	@echo "  docker-build - 构建 Docker 镜像"

# 构建项目
build:
	cargo build

# 运行测试
test:
	cargo test

# 运行服务器
run:
	cargo run --bin iam-core

# 清理构建文件
clean:
	cargo clean

# 生成 OpenAPI 文档
docs:
	@echo "生成 OpenAPI 文档..."
	@mkdir -p doc/api
	@cargo run --bin generate-openapi-docs
	@echo "文档已生成到 doc/api/ 目录"

# 启动 Swagger UI 服务器（用于本地开发）
swagger-ui:
	@echo "启动 Swagger UI 服务器..."
	@echo "访问 http://localhost:8080 查看 API 文档"
	@cd doc/api && python3 -m http.server 8080

# Docker 相关命令
docker:
	docker-compose up -d

docker-build:
	docker build -t iam-core .

docker-stop:
	docker-compose down

# 开发环境设置
dev-setup:
	@echo "设置开发环境..."
	@cp env.example .env
	@echo "请编辑 .env 文件配置环境变量"

# 数据库迁移
migrate:
	cargo run --bin migrate

# 代码格式化
fmt:
	cargo fmt

# 代码检查
clippy:
	cargo clippy

# 完整的开发流程
dev: clean build test docs
	@echo "开发环境准备完成！"
	@echo "运行 'make run' 启动服务器"
	@echo "访问 http://localhost:3000/swagger-ui 查看 API 文档"
