use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::identity_access::commands::RegisterUserCommand;
use crate::error::AppError;
use crate::interface::middleware::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub user_id: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 注册新用户
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), AppError> {
    // 验证输入
    payload.validate()?;

    // 密码哈希
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::DomainError(format!("Password hashing failed: {}", e)))?;

    // 创建命令
    let command = RegisterUserCommand {
        tenant_id: Uuid::new_v4(), // TODO: 从认证上下文获取租户ID
        username: payload.username,
        email: payload.email,
        password_hash,
    };

    // 执行命令
    let user_id = state.user_service.register_user(command).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterUserResponse {
            user_id,
            message: "User registered successfully".to_string(),
        }),
    ))
}

/// 根据ID获取用户信息
pub async fn get_user(
    State(_state): State<AppState>,
    Path(_user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    // TODO: 实现从读模型查询用户
    // 这里需要添加查询服务
    Err(AppError::DomainError("Not implemented yet".to_string()))
}

/// 获取用户列表
pub async fn list_users(
    State(_state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    // TODO: 实现用户列表查询
    Err(AppError::DomainError("Not implemented yet".to_string()))
}
