use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use shared::{
    config::{MicroserviceConfig, ServiceConfig},
    registry::ServiceRegistry,
    tracing::init_tracing,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub service_registry: ServiceRegistry,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = MicroserviceConfig::from_env()?;
    
    // 初始化日志
    init_tracing(&config.service.name)?;

    tracing::info!("启动 {} 微服务", config.service.name);
    tracing::info!("配置: {:?}", config);

    // 创建数据库连接池
    let pool = MySqlPool::connect(&config.database.url).await?;
    tracing::info!("数据库连接成功");

    // 创建服务注册器
    let service_registry = ServiceRegistry::new(
        &config.consul,
        config.service.name.clone(),
        format!("{}-{}", config.service.name, uuid::Uuid::new_v4()),
        config.service.host.clone(),
        config.service.port,
    )?;

    // 注册服务到Consul
    service_registry.register().await?;
    tracing::info!("服务注册到Consul成功");

    let app_state = AppState {
        pool,
        service_registry,
    };

    // 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        .with_state(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("服务器启动在 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "user-service",
        "timestamp": chrono::Utc::now()
    }))
}

async fn list_users(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    // 实现获取用户列表逻辑
    Ok(Json(json!({
        "users": [],
        "total": 0
    })))
}

async fn create_user(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    // 实现创建用户逻辑
    Ok(Json(json!({
        "message": "用户创建成功",
        "id": uuid::Uuid::new_v4()
    })))
}

async fn get_user(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    // 实现获取用户详情逻辑
    Ok(Json(json!({
        "id": 1,
        "username": "testuser",
        "email": "test@example.com"
    })))
}
