use iam_core::{
    application::services::UserService,
    config::AppConfig,
    infrastructure::persistence::{EventStore, SqlxEventStore},
    interface::{middleware::AppState, routes::create_router},
};
use sea_orm::Database;
use sqlx::MySqlPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "iam_core=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    dotenv::dotenv().ok();
    let config = AppConfig::from_env()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;

    tracing::info!("Starting IAM Core server with config: {:?}", config);

    // 连接数据库
    let pool = MySqlPool::connect(&config.database.url).await?;
    let db_conn = Database::connect(&config.database.url).await?;

    tracing::info!("Database connection successful");

    // 运行数据库迁移
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Database migrations completed");

    // 初始化服务
    let event_store = Arc::new(SqlxEventStore::new(pool.clone()));
    let user_service = Arc::new(UserService::new(event_store.clone()));

    // 创建应用状态
    let app_state = AppState::new(user_service, event_store);

    // 创建路由
    let app = create_router(app_state).layer(CorsLayer::permissive());

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await?;

    tracing::info!(
        "Server running on http://{}:{}",
        config.server.host,
        config.server.port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
