# API 文档

IAM Core 系统提供完整的 RESTful API 接口，支持用户管理、认证授权等功能。

## 🔗 基础信息

### 基础 URL
```
开发环境: http://localhost:3000
生产环境: https://your-domain.com
```

### 认证方式
```http
Authorization: Bearer <jwt_token>
```

### 响应格式
所有 API 响应都使用 JSON 格式，包含以下字段：
- `data`: 响应数据
- `error`: 错误信息
- `status`: HTTP 状态码

## 📋 API 端点

### 系统端点

#### 健康检查
```http
GET /health
```

**响应示例:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "0.1.0",
  "environment": "development"
}
```

#### 指标信息
```http
GET /metrics
```

**响应示例:**
```json
{
  "uptime_seconds": 3600,
  "total_requests": 1000,
  "successful_requests": 950,
  "failed_requests": 50,
  "average_response_time_ms": 25.5,
  "active_connections": 10,
  "memory_usage_mb": 128.5
}
```

### 认证端点

#### 用户登录
```http
POST /api/v1/auth/login
```

**请求体:**
```json
{
  "username": "testuser",
  "password": "password123"
}
```

**响应示例:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

#### 刷新令牌
```http
POST /api/v1/auth/refresh
```

**请求头:**
```http
Authorization: Bearer <current_token>
```

#### 用户登出
```http
POST /api/v1/auth/logout
```

**请求头:**
```http
Authorization: Bearer <token>
```

### 用户管理端点

#### 注册用户
```http
POST /api/v1/users
```

**请求体:**
```json
{
  "username": "newuser",
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**响应示例:**
```json
{
  "user_id": "123e4567-e89b-12d3-a456-426614174000",
  "message": "User registered successfully"
}
```

#### 获取用户信息
```http
GET /api/v1/users/{user_id}
```

**请求头:**
```http
Authorization: Bearer <token>
```

**响应示例:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "username": "testuser",
  "email": "test@example.com",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### 获取用户列表
```http
GET /api/v1/users
```

**请求头:**
```http
Authorization: Bearer <token>
```

**查询参数:**
- `limit`: 每页数量 (默认: 100)
- `offset`: 偏移量 (默认: 0)
- `status`: 用户状态过滤

**响应示例:**
```json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "user1",
    "email": "user1@example.com",
    "created_at": "2024-01-01T00:00:00Z"
  },
  {
    "id": "123e4567-e89b-12d3-a456-426614174001",
    "username": "user2",
    "email": "user2@example.com",
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

## 🔒 认证和授权

### JWT Token 结构
```json
{
  "sub": "user_id",
  "username": "testuser",
  "tenant_id": "tenant_id",
  "exp": 1640995200,
  "iat": 1640908800
}
```

### 权限验证
系统使用基于角色的访问控制 (RBAC)：

1. **用户** 被分配到 **角色**
2. **角色** 被授予 **权限**
3. **权限** 控制对资源的访问

### 多租户支持
- 每个请求必须包含租户信息
- 租户信息可以通过以下方式提供：
  - JWT Token 中的 `tenant_id`
  - 请求头 `X-Tenant-ID`
  - 子域名解析

## 📊 状态码

### 成功状态码
- `200 OK`: 请求成功
- `201 Created`: 资源创建成功
- `204 No Content`: 请求成功，无返回内容

### 客户端错误
- `400 Bad Request`: 请求参数错误
- `401 Unauthorized`: 未认证
- `403 Forbidden`: 无权限
- `404 Not Found`: 资源不存在
- `409 Conflict`: 资源冲突
- `422 Unprocessable Entity`: 验证失败

### 服务器错误
- `500 Internal Server Error`: 服务器内部错误
- `502 Bad Gateway`: 网关错误
- `503 Service Unavailable`: 服务不可用

## 🔍 错误处理

### 错误响应格式
```json
{
  "error": "错误描述",
  "status": 400,
  "details": {
    "field": "具体错误信息"
  }
}
```

### 常见错误

#### 验证错误
```json
{
  "error": "username: Username must be at least 3 characters long",
  "status": 400
}
```

#### 认证错误
```json
{
  "error": "Invalid username or password",
  "status": 401
}
```

#### 权限错误
```json
{
  "error": "Insufficient permissions",
  "status": 403
}
```

## 🧪 测试示例

### 使用 curl 测试

#### 1. 注册用户
```bash
curl -X POST http://localhost:3000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

#### 2. 用户登录
```bash
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

#### 3. 获取用户信息
```bash
curl -X GET http://localhost:3000/api/v1/users/{user_id} \
  -H "Authorization: Bearer <token>"
```

### 使用 JavaScript 测试

```javascript
// 注册用户
const registerUser = async (userData) => {
  const response = await fetch('/api/v1/users', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(userData),
  });
  return response.json();
};

// 用户登录
const loginUser = async (credentials) => {
  const response = await fetch('/api/v1/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(credentials),
  });
  return response.json();
};

// 获取用户信息
const getUser = async (userId, token) => {
  const response = await fetch(`/api/v1/users/${userId}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  return response.json();
};
```

## 📝 请求限制

### 频率限制
- 登录接口: 每分钟最多 5 次尝试
- 注册接口: 每分钟最多 3 次尝试
- 其他接口: 每分钟最多 100 次请求

### 请求大小限制
- 最大请求体大小: 10MB
- 最大文件上传: 50MB

## 🔧 开发工具

### API 测试工具
- **Postman**: 图形化 API 测试
- **Insomnia**: 轻量级 API 客户端
- **curl**: 命令行工具
- **HTTPie**: 用户友好的命令行工具

### 文档生成
API 文档可以通过 OpenAPI/Swagger 规范生成，支持：
- 交互式 API 文档
- 代码生成
- 自动化测试

---

**需要更多信息？** 查看 [系统架构文档](./architecture.md) 或 [部署指南](./deployment.md)。
