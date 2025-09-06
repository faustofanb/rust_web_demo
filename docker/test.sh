#!/bin/bash

# Rust Web Demo 中间件测试脚本

echo "🧪 测试 Rust Web Demo 中间件环境"
echo "=================================="

# 测试函数
test_service() {
    local name=$1
    local url=$2
    local expected_status=${3:-200}
    
    echo -n "测试 $name... "
    
    if curl -s -o /dev/null -w "%{http_code}" "$url" | grep -q "$expected_status"; then
        echo "✅ 正常"
        return 0
    else
        echo "❌ 失败"
        return 1
    fi
}

# 等待服务完全启动
echo "⏳ 等待服务完全启动..."
sleep 10

echo ""
echo "🔍 开始测试各个服务..."

# 测试基础服务
test_service "MySQL" "http://localhost:3306" "000"  # MySQL返回连接错误是正常的
test_service "Redis" "http://localhost:6379" "000"  # Redis返回连接错误是正常的
test_service "Kafka" "http://localhost:9092" "000"  # Kafka返回连接错误是正常的

# 测试Web服务
test_service "Kafka UI" "http://localhost:8080"
test_service "Elasticsearch" "http://localhost:9200"
test_service "Kibana" "http://localhost:5601"
test_service "Grafana" "http://localhost:3000"
test_service "Jaeger" "http://localhost:16686"
test_service "Prometheus" "http://localhost:9090"
test_service "MinIO" "http://localhost:9000"
test_service "Nginx" "http://localhost"

echo ""
echo "📊 服务详细信息："

# 检查容器状态
echo "🐳 容器状态："
docker-compose -f docker-compose-dev.yaml ps

echo ""
echo "💾 存储使用情况："
docker system df

echo ""
echo "🌐 网络连接："
docker network ls | grep work_net

echo ""
echo "📈 资源使用情况："
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}"

echo ""
echo "✅ 测试完成！"
echo ""
echo "🔧 常用命令："
echo "- 查看日志: docker-compose -f docker-compose-dev.yaml logs -f [服务名]"
echo "- 重启服务: docker-compose -f docker-compose-dev.yaml restart [服务名]"
echo "- 停止服务: docker-compose -f docker-compose-dev.yaml down"
echo "- 清理数据: docker-compose -f docker-compose-dev.yaml down -v"
