use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::errors::AppResult;
use crate::models::{CreateUserRequest, UserResponse};
// UserService 现在通过 AppState 访问

// 使用 lib.rs 中定义的 AppState
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

pub async fn create_user(
    State(app_state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // 验证请求参数
    request.validate()?;

    let user = app_state.user_service.create_user(request).await?;

    Ok(Json(user))
}

pub async fn get_user(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<UserResponse>> {
    let user = app_state.user_service.get_user_by_id(id).await?;

    Ok(Json(user))
}

pub async fn list_users(
    State(app_state): State<AppState>,
    Query(params): Query<ListUsersQuery>,
) -> AppResult<Json<ListUsersResponse>> {
    let users = app_state
        .user_service
        .list_users(params.limit, params.offset)
        .await?;

    let response = ListUsersResponse {
        total: users.len() as i64,
        limit: params.limit.unwrap_or(10),
        offset: params.offset.unwrap_or(0),
        users,
    };

    Ok(Json(response))
}

pub async fn update_user(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<HashMap<String, String>>,
) -> AppResult<Json<UserResponse>> {
    let username = payload.get("username").map(|s| s.clone());
    let email = payload.get("email").map(|s| s.clone());

    let user = app_state
        .user_service
        .update_user(id, username, email)
        .await?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    app_state.user_service.delete_user(id).await?;

    Ok(Json(serde_json::json!({
        "message": "用户删除成功",
        "id": id
    })))
}
