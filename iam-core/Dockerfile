# 使用官方Rust镜像作为构建环境
FROM rust:1.75-slim as builder

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    default-libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制Cargo文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟main.rs来缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖（利用Docker缓存）
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src
COPY migrations ./migrations

# 构建应用程序
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    default-libmysqlclient21 \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false iamcore

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/iam-core /app/iam-core

# 复制迁移文件
COPY --from=builder /app/migrations /app/migrations

# 更改文件所有者
RUN chown -R iamcore:iamcore /app

# 切换到非root用户
USER iamcore

# 暴露端口
EXPOSE 3000

# 设置环境变量
ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# 启动应用程序
CMD ["./iam-core"]
