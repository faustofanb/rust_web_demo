# Rust Web Demo - 中间件环境

这是一个完整的中间件环境，为Rust Web应用开发提供所需的基础设施服务。

## 🏗️ 包含的中间件

### 数据存储
- **MySQL 8.0** - 关系型数据库
- **Redis 7** - 内存数据库/缓存
- **Elasticsearch 8.11** - 搜索引擎
- **MinIO** - 对象存储

### 消息队列
- **Kafka 7.4** - 分布式流处理平台
- **Zookeeper** - Kafka协调服务

### 监控与追踪
- **Prometheus** - 监控指标收集
- **Grafana** - 监控数据可视化
- **Jaeger** - 分布式链路追踪

### 日志处理
- **Logstash** - 日志处理管道
- **Kibana** - 日志可视化

### 管理界面
- **Kafka UI** - Kafka集群管理
- **Nginx** - 反向代理和统一入口

## 🚀 快速开始

### 1. 启动环境

```bash
# 进入docker目录
cd docker

# 启动所有服务
./start.sh

# 或者使用docker-compose
docker-compose -f docker-compose-dev.yaml up -d
```

### 2. 测试环境

```bash
# 运行测试脚本
./test.sh
```

### 3. 访问服务

| 服务 | 访问地址 | 用户名/密码 | 说明 |
|------|----------|-------------|------|
| **统一入口** | http://localhost | - | 所有服务的统一入口 |
| **Kafka UI** | http://localhost/kafka/ | - | Kafka集群管理 |
| **Kibana** | http://localhost/kibana/ | - | 日志和数据分析 |
| **Grafana** | http://localhost/grafana/ | admin/admin123 | 监控仪表板 |
| **Jaeger** | http://localhost/jaeger/ | - | 链路追踪 |
| **Prometheus** | http://localhost/prometheus/ | - | 监控指标 |
| **MinIO Console** | http://localhost:9001 | minioadmin/minioadmin123 | 对象存储管理 |

### 4. 直接访问

| 服务 | 地址 | 用户名/密码 | 说明 |
|------|------|-------------|------|
| **MySQL** | localhost:3306 | root/123456 | 数据库 |
| **Redis** | localhost:6379 | - | 缓存 |
| **Kafka** | localhost:9092 | - | 消息队列 |
| **Elasticsearch** | localhost:9200 | - | 搜索引擎 |
| **MinIO API** | localhost:9000 | minioadmin/minioadmin123 | 对象存储API |

## 🔧 配置说明

### 环境变量

所有服务都配置了以下环境变量：
- `TZ=Asia/Shanghai` - 时区设置

### 资源限制

每个服务都配置了内存限制，适合开发环境：
- MySQL: 1GB
- Kafka: 1GB
- Elasticsearch: 1GB
- 其他服务: 128MB-512MB

### 数据持久化

以下数据会被持久化保存：
- MySQL数据
- Redis数据
- Kafka数据
- Elasticsearch数据
- MinIO数据
- Prometheus数据
- Grafana配置

## 📊 监控配置

### Prometheus监控目标

- 应用监控 (Rust应用)
- MySQL监控
- Redis监控
- Kafka监控
- Elasticsearch监控
- Nginx监控
- 系统监控 (Node Exporter)
- 容器监控 (cAdvisor)

### Grafana仪表板

- 系统监控仪表板
- 应用监控仪表板
- 数据库监控仪表板
- 消息队列监控仪表板

## 🔍 日志配置

### Logstash管道

- 接收Beats输入 (端口5044)
- 接收TCP/UDP输入 (端口5000)
- 解析JSON日志
- 输出到Elasticsearch

### 日志索引

- 索引格式: `logstash-YYYY.MM.DD`
- 自动创建索引
- 7天保留期

## 🛠️ 开发集成

### Rust应用配置

在您的Rust应用中，可以使用以下配置连接到中间件：

```rust
// 数据库配置
DATABASE_URL=mysql://root:123456@localhost:3306/your_database

// Redis配置
REDIS_URL=redis://localhost:6379

// Kafka配置
KAFKA_BROKERS=localhost:9092

// Elasticsearch配置
ELASTICSEARCH_URL=http://localhost:9200

// MinIO配置
MINIO_ENDPOINT=localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin123
```

### 健康检查

所有服务都提供健康检查端点：
- Nginx: `GET /health`
- 其他服务: 各自的健康检查端点

## 📝 常用命令

### 服务管理

```bash
# 启动所有服务
docker-compose -f docker-compose-dev.yaml up -d

# 停止所有服务
docker-compose -f docker-compose-dev.yaml down

# 重启特定服务
docker-compose -f docker-compose-dev.yaml restart [服务名]

# 查看服务状态
docker-compose -f docker-compose-dev.yaml ps

# 查看服务日志
docker-compose -f docker-compose-dev.yaml logs -f [服务名]
```

### 数据管理

```bash
# 清理所有数据
docker-compose -f docker-compose-dev.yaml down -v

# 备份数据
docker run --rm -v [volume_name]:/data -v $(pwd):/backup alpine tar czf /backup/backup.tar.gz /data

# 恢复数据
docker run --rm -v [volume_name]:/data -v $(pwd):/backup alpine tar xzf /backup/backup.tar.gz -C /
```

### 监控命令

```bash
# 查看资源使用情况
docker stats

# 查看网络连接
docker network ls

# 查看存储使用
docker system df
```

## 🚨 故障排除

### 常见问题

1. **端口冲突**
   - 检查端口是否被占用: `lsof -i :端口号`
   - 修改docker-compose-dev.yaml中的端口映射

2. **内存不足**
   - 调整服务的内存限制
   - 关闭不必要的服务

3. **服务启动失败**
   - 查看服务日志: `docker-compose logs [服务名]`
   - 检查配置文件是否正确

4. **数据丢失**
   - 检查volume挂载是否正确
   - 确认数据目录权限

### 日志位置

- 容器日志: `docker-compose logs [服务名]`
- 应用日志: 通过Logstash收集到Elasticsearch
- 系统日志: 通过cAdvisor收集

## 🔄 更新升级

### 更新服务版本

1. 修改docker-compose-dev.yaml中的镜像版本
2. 重新拉取镜像: `docker-compose pull`
3. 重启服务: `docker-compose up -d`

### 备份升级

1. 备份数据: `docker-compose down -v` 前先备份volumes
2. 更新配置
3. 启动新版本
4. 验证数据完整性

## 📚 相关文档

- [Docker Compose文档](https://docs.docker.com/compose/)
- [Kafka文档](https://kafka.apache.org/documentation/)
- [Elasticsearch文档](https://www.elastic.co/guide/en/elasticsearch/reference/current/)
- [Prometheus文档](https://prometheus.io/docs/)
- [Grafana文档](https://grafana.com/docs/)
- [Jaeger文档](https://www.jaegertracing.io/docs/)
