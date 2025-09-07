use utoipa::OpenApi;

use crate::interface::handlers::{auth_handler, user_handler};

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handler::register_user,
        user_handler::get_user,
        user_handler::list_users,
        auth_handler::login,
        auth_handler::refresh_token,
        auth_handler::logout,
        health_check
    ),
    components(
        schemas(
            user_handler::RegisterUserRequest,
            user_handler::UserResponse,
            user_handler::RegisterUserResponse,
            auth_handler::LoginRequest,
            auth_handler::LoginResponse,
            auth_handler::TokenInfo,
            HealthResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "users", description = "用户管理相关接口"),
        (name = "auth", description = "认证相关接口"),
        (name = "system", description = "系统相关接口")
    ),
    info(
        title = "IAM Core API",
        version = "0.1.0",
        description = "身份与访问管理核心系统 API 文档",
        contact(
            name = "IAM Core Team",
            email = "team@iamcore.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "开发环境"),
        (url = "https://api.iamcore.com", description = "生产环境")
    )
)]
pub struct ApiDoc;

use serde::{Deserialize, Serialize};

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 时间戳
    pub timestamp: String,
    /// 版本号
    pub version: String,
    /// 环境
    pub environment: String,
}

/// 错误响应
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    /// 错误信息
    pub error: String,
    /// HTTP 状态码
    pub status: u16,
    /// 详细信息
    pub details: Option<serde_json::Value>,
}

/// 健康检查端点
#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses(
        (status = 200, description = "服务健康", body = HealthResponse)
    )
)]
pub async fn health_check() -> axum::Json<HealthResponse> {
    use chrono::Utc;
    
    axum::Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
    })
}
