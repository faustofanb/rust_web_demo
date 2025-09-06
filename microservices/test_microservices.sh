#!/bin/bash

# Rust微服务测试脚本

echo "🚀 开始测试 Rust 微服务架构"
echo "=================================="

# 等待服务启动
echo "等待服务启动..."
sleep 30

# 1. 检查Consul健康状态
echo "1. 检查Consul健康状态"
curl -s "http://localhost:8500/v1/status/leader" | jq '.' || echo "Consul检查失败"
echo ""

# 2. 检查注册的服务
echo "2. 检查注册的服务"
curl -s "http://localhost:8500/v1/catalog/services" | jq '.' || echo "服务列表获取失败"
echo ""

# 3. 测试网关健康检查
echo "3. 测试网关健康检查"
curl -s "http://localhost:8080/health" | jq '.' || echo "网关健康检查失败"
echo ""

# 4. 测试用户服务健康检查
echo "4. 测试用户服务健康检查"
curl -s "http://localhost:8081/health" | jq '.' || echo "用户服务健康检查失败"
echo ""

# 5. 通过网关访问用户服务
echo "5. 通过网关访问用户服务"
curl -s "http://localhost:8080/api/users" | jq '.' || echo "网关代理失败"
echo ""

# 6. 直接访问用户服务
echo "6. 直接访问用户服务"
curl -s "http://localhost:8081/api/users" | jq '.' || echo "直接访问用户服务失败"
echo ""

# 7. 测试服务发现
echo "7. 测试服务发现"
curl -s "http://localhost:8500/v1/health/service/user-service" | jq '.' || echo "服务发现检查失败"
echo ""

# 8. 检查Jaeger追踪
echo "8. 检查Jaeger追踪"
curl -s "http://localhost:16686/api/services" | jq '.' || echo "Jaeger检查失败"
echo ""

# 9. 检查Prometheus指标
echo "9. 检查Prometheus指标"
curl -s "http://localhost:9090/api/v1/targets" | jq '.' || echo "Prometheus检查失败"
echo ""

# 10. 检查Grafana
echo "10. 检查Grafana"
curl -s "http://localhost:3000/api/health" | jq '.' || echo "Grafana检查失败"
echo ""

echo "✅ 微服务测试完成"
echo ""
echo "📊 服务访问地址："
echo "- 网关: http://localhost:8080"
echo "- 用户服务: http://localhost:8081"
echo "- Consul UI: http://localhost:8500"
echo "- Jaeger UI: http://localhost:16686"
echo "- Prometheus: http://localhost:9090"
echo "- Grafana: http://localhost:3000 (admin/admin)"
