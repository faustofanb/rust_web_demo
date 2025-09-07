use axum::{
    routing::{get, post},
    Router,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use crate::interface::handlers::{auth_handler, user_handler};
use crate::interface::middleware::AppState;
use crate::openapi::{ApiDoc, health_check};

/// 创建应用程序路由
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", create_api_routes())
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .with_state(state)
}

/// 创建API路由
fn create_api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", create_auth_routes())
        .nest("/users", create_user_routes())
}

/// 创建认证相关路由
fn create_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth_handler::login))
        .route("/refresh", post(auth_handler::refresh_token))
        .route("/logout", post(auth_handler::logout))
}

/// 创建用户相关路由
fn create_user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(user_handler::register_user))
        .route("/", get(user_handler::list_users))
        .route("/:id", get(user_handler::get_user))
}

// 健康检查端点现在在 openapi 模块中定义
