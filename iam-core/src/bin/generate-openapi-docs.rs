use std::fs;
use std::path::Path;

use iam_core::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() {
    // 创建 doc/api 目录
    let doc_dir = Path::new("doc/api");
    if !doc_dir.exists() {
        fs::create_dir_all(doc_dir).expect("Failed to create doc/api directory");
    }

    // 生成 OpenAPI JSON 文档
    let openapi_json = serde_json::to_string_pretty(&ApiDoc::openapi())
        .expect("Failed to serialize OpenAPI spec");

    // 写入 JSON 文件
    let json_path = doc_dir.join("openapi.json");
    fs::write(&json_path, openapi_json)
        .expect("Failed to write openapi.json");

    println!("✅ OpenAPI JSON documentation generated at: {}", json_path.display());

    // 生成 YAML 文档
    let openapi_yaml = serde_yaml::to_string(&ApiDoc::openapi())
        .expect("Failed to serialize OpenAPI spec to YAML");

    let yaml_path = doc_dir.join("openapi.yaml");
    fs::write(&yaml_path, openapi_yaml)
        .expect("Failed to write openapi.yaml");

    println!("✅ OpenAPI YAML documentation generated at: {}", yaml_path.display());

    // 生成 HTML 文档
    let html_content = generate_html_docs();
    let html_path = doc_dir.join("index.html");
    fs::write(&html_path, html_content)
        .expect("Failed to write index.html");

    println!("✅ OpenAPI HTML documentation generated at: {}", html_path.display());

    // 生成 README
    let readme_content = generate_readme();
    let readme_path = doc_dir.join("README.md");
    fs::write(&readme_path, readme_content)
        .expect("Failed to write README.md");

    println!("✅ API documentation README generated at: {}", readme_path.display());

    println!("\n🎉 所有 API 文档已生成完成！");
    println!("📁 文档位置: doc/api/");
    println!("🌐 在线查看: 运行 'make swagger-ui' 或直接打开 doc/api/index.html");
}

fn generate_html_docs() -> String {
    format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>IAM Core API 文档</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        html {{
            box-sizing: border-box;
            overflow: -moz-scrollbars-vertical;
            overflow-y: scroll;
        }}
        *, *:before, *:after {{
            box-sizing: inherit;
        }}
        body {{
            margin:0;
            background: #fafafa;
        }}
        .swagger-ui .topbar {{
            background-color: #2c3e50;
        }}
        .swagger-ui .topbar .download-url-wrapper {{
            display: none;
        }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {{
            const ui = SwaggerUIBundle({{
                url: './openapi.json',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout",
                validatorUrl: null,
                tryItOutEnabled: true,
                supportedSubmitMethods: ['get', 'post', 'put', 'delete', 'patch'],
                docExpansion: 'list',
                defaultModelsExpandDepth: 3,
                defaultModelExpandDepth: 3,
                displayRequestDuration: true,
                showExtensions: true,
                showCommonExtensions: true,
                onComplete: function() {{
                    console.log('IAM Core API 文档加载完成');
                }}
            }});
        }};
    </script>
</body>
</html>"#)
}

fn generate_readme() -> String {
    format!(r#"# IAM Core API 文档

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
- `GET /api/v1/users/{{id}}` - 获取用户信息

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
"#)
}
