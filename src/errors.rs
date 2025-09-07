use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("数据库迁移错误: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("JWT错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("验证错误: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("HTTP客户端错误: {0}")]
    Http(#[from] reqwest::Error),

    #[error("用户未找到")]
    UserNotFound,

    #[error("认证失败")]
    AuthenticationFailed,

    #[error("权限不足")]
    InsufficientPermissions,

    #[error("内部服务器错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "用户未找到"),
            AppError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "认证失败"),
            AppError::InsufficientPermissions => (StatusCode::FORBIDDEN, "权限不足"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "请求参数验证失败"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "数据库操作失败"),
            AppError::Migration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "数据库迁移失败"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "配置错误"),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "缓存服务错误"),
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Token验证失败"),
            AppError::Http(_) => (StatusCode::BAD_GATEWAY, "外部服务调用失败"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "内部服务器错误"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string(),
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
