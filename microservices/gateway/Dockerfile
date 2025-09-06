# 多阶段构建
FROM rust:1.75 as builder

# 设置工作目录
WORKDIR /app

# 复制共享库
COPY shared/ ./shared/

# 复制网关代码
COPY gateway/ ./gateway/

# 设置工作目录为网关
WORKDIR /app/gateway

# 构建应用
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

# 安装必要的运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false rustuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/gateway/target/release/gateway /app/gateway

# 更改文件所有者
RUN chown rustuser:rustuser /app/gateway

# 切换到非root用户
USER rustuser

# 暴露端口
EXPOSE 8080

# 启动应用
CMD ["./gateway"]
