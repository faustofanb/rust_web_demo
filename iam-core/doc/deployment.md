# 部署指南

本指南介绍如何在不同环境中部署 IAM Core 系统。

## 🏗️ 部署架构

### 生产环境架构

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │────│   Nginx Proxy   │────│   IAM Core      │
│   (HAProxy)     │    │                 │    │   Application   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                        │
                                │                        │
                       ┌─────────────────┐    ┌─────────────────┐
                       │     Redis       │    │     MySQL       │
                       │    (Cache)      │    │   (Database)    │
                       └─────────────────┘    └─────────────────┘
```

## 🐳 Docker 部署

### 使用 Docker Compose

#### 1. 准备环境

```bash
# 克隆项目
git clone <repository-url>
cd iam-core

# 复制环境配置
cp env.example .env
```

#### 2. 配置环境变量

```env
# 生产环境配置
DATABASE_URL=mysql://iamuser:strongpassword@mysql:3306/iam_core
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
JWT_SECRET=your-super-secure-jwt-secret-key-here
JWT_EXPIRATION_HOURS=24
ENVIRONMENT=production
CORS_ORIGINS=https://yourdomain.com,https://api.yourdomain.com
```

#### 3. 启动服务

```bash
# 启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f iam-core
```

#### 4. 验证部署

```bash
# 检查健康状态
curl http://localhost/health

# 检查服务状态
docker-compose exec iam-core curl http://localhost:3000/health
```

### 单独构建 Docker 镜像

#### 1. 构建镜像

```bash
# 构建生产镜像
docker build -t iam-core:latest .

# 构建特定版本
docker build -t iam-core:v1.0.0 .
```

#### 2. 运行容器

```bash
# 运行容器
docker run -d \
  --name iam-core \
  -p 3000:3000 \
  --env-file .env \
  --network iam-network \
  iam-core:latest

# 查看容器状态
docker ps
docker logs iam-core
```

## ☸️ Kubernetes 部署

### 1. 创建命名空间

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: iam-core
```

### 2. 配置 ConfigMap

```yaml
# k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: iam-core-config
  namespace: iam-core
data:
  DATABASE_URL: "mysql://iamuser:password@mysql-service:3306/iam_core"
  SERVER_HOST: "0.0.0.0"
  SERVER_PORT: "3000"
  ENVIRONMENT: "production"
```

### 3. 配置 Secret

```yaml
# k8s/secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: iam-core-secret
  namespace: iam-core
type: Opaque
data:
  JWT_SECRET: <base64-encoded-secret>
  DATABASE_PASSWORD: <base64-encoded-password>
```

### 4. 部署应用

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: iam-core
  namespace: iam-core
spec:
  replicas: 3
  selector:
    matchLabels:
      app: iam-core
  template:
    metadata:
      labels:
        app: iam-core
    spec:
      containers:
      - name: iam-core
        image: iam-core:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            configMapKeyRef:
              name: iam-core-config
              key: DATABASE_URL
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: iam-core-secret
              key: JWT_SECRET
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
```

### 5. 创建服务

```yaml
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: iam-core-service
  namespace: iam-core
spec:
  selector:
    app: iam-core
  ports:
  - port: 80
    targetPort: 3000
  type: ClusterIP
```

### 6. 部署到 Kubernetes

```bash
# 应用所有配置
kubectl apply -f k8s/

# 查看部署状态
kubectl get pods -n iam-core
kubectl get services -n iam-core

# 查看日志
kubectl logs -f deployment/iam-core -n iam-core
```

## 🖥️ 传统服务器部署

### 1. 系统要求

- **操作系统**: Ubuntu 20.04+ / CentOS 8+ / RHEL 8+
- **内存**: 最少 2GB，推荐 4GB+
- **CPU**: 最少 2 核心，推荐 4 核心+
- **存储**: 最少 20GB，推荐 50GB+

### 2. 安装依赖

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y curl build-essential pkg-config libssl-dev

# CentOS/RHEL
sudo yum update
sudo yum groupinstall -y "Development Tools"
sudo yum install -y openssl-devel pkgconfig
```

### 3. 安装 Rust

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 验证安装
rustc --version
cargo --version
```

### 4. 安装 MySQL

```bash
# Ubuntu/Debian
sudo apt install -y mysql-server
sudo systemctl start mysql
sudo systemctl enable mysql

# CentOS/RHEL
sudo yum install -y mysql-server
sudo systemctl start mysqld
sudo systemctl enable mysqld
```

### 5. 配置数据库

```bash
# 创建数据库和用户
mysql -u root -p
```

```sql
CREATE DATABASE iam_core;
CREATE USER 'iamuser'@'localhost' IDENTIFIED BY 'strongpassword';
GRANT ALL PRIVILEGES ON iam_core.* TO 'iamuser'@'localhost';
FLUSH PRIVILEGES;
EXIT;
```

### 6. 部署应用

```bash
# 克隆项目
git clone <repository-url>
cd iam-core

# 构建应用
cargo build --release

# 创建配置文件
cp env.example .env
vim .env

# 运行数据库迁移
cargo run --bin migrate

# 启动应用
cargo run --release
```

### 7. 配置系统服务

```ini
# /etc/systemd/system/iam-core.service
[Unit]
Description=IAM Core Service
After=network.target mysql.service

[Service]
Type=simple
User=iam-core
WorkingDirectory=/opt/iam-core
ExecStart=/opt/iam-core/target/release/iam-core
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

```bash
# 创建用户
sudo useradd -r -s /bin/false iam-core

# 复制文件
sudo cp -r . /opt/iam-core
sudo chown -R iam-core:iam-core /opt/iam-core

# 启用服务
sudo systemctl daemon-reload
sudo systemctl enable iam-core
sudo systemctl start iam-core

# 查看状态
sudo systemctl status iam-core
```

## 🔧 环境配置

### 开发环境

```env
ENVIRONMENT=development
RUST_LOG=debug
DATABASE_URL=mysql://root:password@localhost:3306/iam_core_dev
JWT_SECRET=dev-secret-key
CORS_ORIGINS=http://localhost:3000,http://localhost:3001
```

### 测试环境

```env
ENVIRONMENT=test
RUST_LOG=info
DATABASE_URL=mysql://root:password@localhost:3306/iam_core_test
JWT_SECRET=test-secret-key
CORS_ORIGINS=http://test.yourdomain.com
```

### 生产环境

```env
ENVIRONMENT=production
RUST_LOG=warn
DATABASE_URL=mysql://iamuser:strongpassword@mysql-cluster:3306/iam_core
JWT_SECRET=your-super-secure-jwt-secret-key-here
CORS_ORIGINS=https://yourdomain.com,https://api.yourdomain.com
```

## 📊 监控和日志

### 日志配置

```bash
# 使用 systemd journal
sudo journalctl -u iam-core -f

# 使用 logrotate
sudo vim /etc/logrotate.d/iam-core
```

```bash
# /etc/logrotate.d/iam-core
/opt/iam-core/logs/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 iam-core iam-core
    postrotate
        systemctl reload iam-core
    endscript
}
```

### 监控配置

```bash
# 安装 Prometheus Node Exporter
wget https://github.com/prometheus/node_exporter/releases/download/v1.3.1/node_exporter-1.3.1.linux-amd64.tar.gz
tar xvfz node_exporter-1.3.1.linux-amd64.tar.gz
sudo mv node_exporter-1.3.1.linux-amd64/node_exporter /usr/local/bin/
```

### 健康检查

```bash
# 创建健康检查脚本
cat > /opt/iam-core/health-check.sh << 'EOF'
#!/bin/bash
curl -f http://localhost:3000/health || exit 1
EOF

chmod +x /opt/iam-core/health-check.sh

# 添加到 crontab
echo "*/5 * * * * /opt/iam-core/health-check.sh" | crontab -
```

## 🔒 安全配置

### SSL/TLS 配置

```nginx
# Nginx SSL 配置
server {
    listen 443 ssl http2;
    server_name yourdomain.com;
  
    ssl_certificate /path/to/certificate.crt;
    ssl_certificate_key /path/to/private.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;
  
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 防火墙配置

```bash
# UFW (Ubuntu)
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable

# firewalld (CentOS/RHEL)
sudo firewall-cmd --permanent --add-service=ssh
sudo firewall-cmd --permanent --add-service=http
sudo firewall-cmd --permanent --add-service=https
sudo firewall-cmd --reload
```

## 🚀 性能优化

### 数据库优化

```sql
-- 创建索引
CREATE INDEX idx_users_tenant_id ON users_view (tenant_id);
CREATE INDEX idx_users_username ON users_view (username);
CREATE INDEX idx_events_aggregate_id ON events (aggregate_id);

-- 优化查询
EXPLAIN SELECT * FROM users_view WHERE tenant_id = ? AND status = 'active';
```

### 应用优化

```bash
# 设置环境变量
export RUST_LOG=warn
export RUST_BACKTRACE=1

# 使用 release 构建
cargo build --release

# 设置 CPU 亲和性
taskset -c 0,1 /opt/iam-core/target/release/iam-core
```

## 🔄 更新和回滚

### 滚动更新

```bash
# Docker Compose
docker-compose pull
docker-compose up -d --no-deps iam-core

# Kubernetes
kubectl set image deployment/iam-core iam-core=iam-core:v1.1.0 -n iam-core
kubectl rollout status deployment/iam-core -n iam-core
```

### 回滚

```bash
# Docker Compose
docker-compose down
docker-compose up -d

# Kubernetes
kubectl rollout undo deployment/iam-core -n iam-core
kubectl rollout status deployment/iam-core -n iam-core
```

## 📋 部署检查清单

### 部署前检查

- [ ] 环境变量配置正确
- [ ] 数据库连接正常
- [ ] SSL 证书有效
- [ ] 防火墙规则配置
- [ ] 监控系统就绪
- [ ] 备份策略制定

### 部署后验证

- [ ] 健康检查通过
- [ ] API 接口正常
- [ ] 数据库连接正常
- [ ] 日志输出正常
- [ ] 监控指标正常
- [ ] 性能测试通过

---

**需要帮助？** 查看 [监控运维文档](./monitoring.md) 或提交 Issue。
