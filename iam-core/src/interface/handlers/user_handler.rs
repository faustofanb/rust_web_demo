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

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct RegisterUserRequest {
    /// 用户名，3-50个字符
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    /// 邮箱地址
    #[validate(email)]
    pub email: String,
    /// 密码，至少8个字符
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RegisterUserResponse {
    /// 用户ID
    pub user_id: Uuid,
    /// 响应消息
    pub message: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    /// 用户ID
    pub id: Uuid,
    /// 用户名
    pub username: String,
    /// 邮箱地址
    pub email: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 注册新用户
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "users",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "用户注册成功", body = RegisterUserResponse),
        (status = 400, description = "请求参数错误"),
        (status = 409, description = "用户名或邮箱已存在"),
        (status = 500, description = "服务器内部错误")
    )
)]
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
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}",
    tag = "users",
    params(
        ("user_id" = Uuid, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "获取用户信息成功", body = UserResponse),
        (status = 401, description = "未认证"),
        (status = 403, description = "无权限"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.query_service
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
    }))
}

/// 获取用户列表
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    responses(
        (status = 200, description = "获取用户列表成功", body = Vec<UserResponse>),
        (status = 401, description = "未认证"),
        (status = 403, description = "无权限"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    // TODO: 从请求中获取租户ID和分页参数
    let tenant_id = uuid::Uuid::new_v4(); // 临时使用随机UUID
    let users = state.query_service
        .get_users_by_tenant(tenant_id, Some(100), Some(0))
        .await?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        })
        .collect();

    Ok(Json(user_responses))
}
