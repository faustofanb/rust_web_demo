use axum::{extract::State, response::Json};
use serde_json::{json, Value};

use crate::errors::AppResult;
use crate::AppState;

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "message": "服务运行正常",
        "timestamp": chrono::Utc::now()
    }))
}

pub async fn readiness_check(State(app_state): State<AppState>) -> AppResult<Json<Value>> {
    // 检查数据库连接 - 通过 user_repository 访问数据库
    // 这里我们可以通过调用一个简单的数据库操作来检查连接
    let _ = app_state.user_service.list_users(Some(1), Some(0)).await?;

    Ok(Json(json!({
        "status": "ready",
        "message": "服务就绪",
        "database": "connected",
        "timestamp": chrono::Utc::now()
    })))
}
