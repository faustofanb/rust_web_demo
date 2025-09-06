#!/bin/bash

# Rust Web Demo 中间件启动脚本

echo "🚀 启动 Rust Web Demo 中间件环境"
echo "=================================="

# 检查Docker是否运行
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker未运行，请先启动Docker"
    exit 1
fi

# 检查Docker Compose是否安装
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose未安装"
    exit 1
fi

# 创建必要的目录
echo "📁 创建必要的目录..."
mkdir -p logs
mkdir -p data

# 启动服务
echo "🐳 启动Docker容器..."
docker-compose -f docker-compose-dev.yaml up -d

# 等待服务启动
echo "⏳ 等待服务启动..."
sleep 30

# 检查服务状态
echo "🔍 检查服务状态..."
docker-compose -f docker-compose-dev.yaml ps

echo ""
echo "✅ 中间件环境启动完成！"
echo ""
echo "📊 服务访问地址："
echo "- 统一入口: http://localhost"
echo "- Kafka UI: http://localhost/kafka/"
echo "- Kibana: http://localhost/kibana/"
echo "- Grafana: http://localhost/grafana/ (admin/admin123)"
echo "- Jaeger: http://localhost/jaeger/"
echo "- Prometheus: http://localhost/prometheus/"
echo "- MinIO Console: http://localhost:9001 (minioadmin/minioadmin123)"
echo ""
echo "🔧 直接访问地址："
echo "- MySQL: localhost:3306 (root/123456)"
echo "- Redis: localhost:6379"
echo "- Kafka: localhost:9092"
echo "- Elasticsearch: localhost:9200"
echo ""
echo "📝 查看日志: docker-compose -f docker-compose-dev.yaml logs -f [服务名]"
echo "🛑 停止服务: docker-compose -f docker-compose-dev.yaml down"
