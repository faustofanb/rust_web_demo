use axum::{extract::State, response::Json};
use serde_json::Value;
use validator::Validate;

use crate::errors::AppResult;
use crate::models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse};
// AuthService 现在通过 AppState 访问

// 使用 lib.rs 中定义的 AppState
use crate::AppState;

pub async fn register(
    State(app_state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // 验证请求参数
    request.validate()?;

    let user = app_state.auth_service
        .register(&request.username, &request.email, &request.password)
        .await?;

    Ok(Json(user))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    // 验证请求参数
    request.validate()?;

    let response = app_state.auth_service.login(request).await?;

    Ok(Json(response))
}

pub async fn me(
    State(app_state): State<AppState>,
    Json(payload): Json<Value>,
) -> AppResult<Json<UserResponse>> {
    let token = payload
        .get("token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| crate::errors::AppError::AuthenticationFailed)?;

    let user = app_state.auth_service.get_user_from_token(token).await?;

    Ok(Json(user))
}
