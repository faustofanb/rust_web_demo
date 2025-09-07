use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::AppError;
use crate::interface::middleware::{AppState, auth::generate_token};

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct LoginRequest {
    /// 用户名
    #[validate(length(min = 3))]
    pub username: String,
    /// 密码
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TokenInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub username: String,
    /// 过期时间戳
    pub exp: u64,
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = LoginResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "用户名或密码错误"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    // 验证输入
    payload.validate()?;

    // 1. 验证用户凭据（这里需要从请求中获取租户ID，暂时使用默认值）
    // TODO: 在实际应用中，租户ID应该从请求头或子域名中获取
    let tenant_id = uuid::Uuid::new_v4(); // 临时使用随机UUID
    
    let user = state.query_service
        .validate_user_credentials(&payload.username, &payload.password, tenant_id)
        .await?
        .ok_or_else(|| AppError::AuthenticationError("Invalid username or password".to_string()))?;

    // 4. 生成JWT token
    let token = generate_token(
        user.id,
        user.username,
        user.tenant_id,
        &state.config.jwt.secret,
        state.config.jwt.expiration_hours,
    )?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: state.config.jwt.expiration_hours * 3600,
        }),
    ))
}

/// 刷新token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    responses(
        (status = 200, description = "令牌刷新成功", body = LoginResponse),
        (status = 401, description = "无效的令牌"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn refresh_token(
    State(_state): State<AppState>,
) -> Result<Json<LoginResponse>, AppError> {
    // TODO: 实现token刷新逻辑
    Err(AppError::DomainError("Token refresh not implemented yet".to_string()))
}

/// 用户登出
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "登出成功"),
        (status = 401, description = "未认证"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn logout(
    State(_state): State<AppState>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // TODO: 实现登出逻辑（将token加入黑名单等）
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Logged out successfully"
        })),
    ))
}
