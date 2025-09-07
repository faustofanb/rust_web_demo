pub mod config;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use crate::{
    handlers::{
        auth_handlers::{login, me, register},
        health_handlers::{health_check, readiness_check},
        user_handlers::{create_user, delete_user, get_user, list_users, update_user},
    },
    middleware::cors::cors_layer,
    repositories::UserRepository,
    services::{AuthService, UserService},
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::MySqlPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

// 重新导出常用的类型
pub use config::AppConfig;
pub use errors::{AppError, AppResult};

// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub auth_service: services::AuthService,
    pub user_service: services::UserService,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Result<Self, AppError> {
        // 创建数据库连接池
        let pool = MySqlPool::connect(&config.database.url).await?;

        // 运行数据库迁移
        sqlx::migrate!("./migrations").run(&pool).await?;

        // 初始化服务
        let user_repository = UserRepository::new(pool);
        let auth_service = AuthService::new(
            user_repository.clone(),
            config.jwt.secret.clone(),
            config.jwt.expiration,
        );
        let user_service = UserService::new(user_repository, auth_service.clone());

        Ok(AppState {
            auth_service,
            user_service,
        })
    }
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter("rust_web_demo=debug,tower_http=debug")
        .init();
}

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // 健康检查
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        // 认证路由
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/me", post(me))
        // 用户管理路由
        .route("/api/users", post(create_user))
        .route("/api/users", get(list_users))
        .route("/api/users/:id", get(get_user))
        .route("/api/users/:id", put(update_user))
        .route("/api/users/:id", delete(delete_user))
        // 中间件
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer()),
        )
        .with_state(app_state)
}
