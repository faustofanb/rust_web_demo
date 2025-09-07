# IAM Core API 文档

这是 IAM Core 系统的 API 文档，包含所有可用的接口和数据结构。

## 📁 文件说明

- `openapi.json` - OpenAPI 3.0 规范的 JSON 格式
- `openapi.yaml` - OpenAPI 3.0 规范的 YAML 格式  
- `index.html` - 交互式 Swagger UI 文档
- `README.md` - 本说明文件

## 🌐 查看文档

### 方式一：直接打开 HTML 文件
直接双击 `index.html` 文件在浏览器中打开。

### 方式二：使用本地服务器
```bash
# 在 doc/api 目录下启动 HTTP 服务器
cd doc/api
python3 -m http.server 8080

# 然后访问 http://localhost:8080
```

### 方式三：使用项目 Makefile
```bash
# 启动 Swagger UI 服务器
make swagger-ui
```

### 方式四：访问运行中的应用
如果应用正在运行，可以访问：
- Swagger UI: http://localhost:3000/swagger-ui
- OpenAPI JSON: http://localhost:3000/api-docs/openapi.json

## 🔧 重新生成文档

当 API 发生变化时，需要重新生成文档：

```bash
# 使用 Makefile
make docs

# 或直接运行二进制文件
cargo run --bin generate-openapi-docs
```

## 📋 API 概览

### 认证接口
- `POST /api/v1/auth/login` - 用户登录
- `POST /api/v1/auth/refresh` - 刷新令牌
- `POST /api/v1/auth/logout` - 用户登出

### 用户管理接口
- `POST /api/v1/users` - 注册用户
- `GET /api/v1/users` - 获取用户列表
- `GET /api/v1/users/{id}` - 获取用户信息

### 系统接口
- `GET /health` - 健康检查

## 🛠️ 开发工具

### Postman 导入
可以将 `openapi.json` 文件导入到 Postman 中：
1. 打开 Postman
2. 点击 Import
3. 选择 `openapi.json` 文件
4. 确认导入

### Insomnia 导入
可以将 `openapi.yaml` 文件导入到 Insomnia 中：
1. 打开 Insomnia
2. 点击 Create
3. 选择 Import from File
4. 选择 `openapi.yaml` 文件

## 📝 更新日志

- **v0.1.0** - 初始版本，包含基础的用户管理和认证接口

---

**注意**: 本文档由代码自动生成，请勿手动修改。如需更新，请修改源代码中的 OpenAPI 注解。
