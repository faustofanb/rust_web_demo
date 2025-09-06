// 使用lib.rs中定义的模块
use rust_web_demo::{
    config::AppConfig,
    handlers::{
        auth_handlers::{login, me, register},
        health_handlers::{health_check, readiness_check},
        user_handlers::{create_user, delete_user, get_user, list_users, update_user},
    },
    middleware::cors::cors_layer,
    repositories::UserRepository,
    services::{AuthService, UserService},
    AppState,
};

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;


// AppState 现在在 lib.rs 中定义

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = AppConfig::from_env()?;
    
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("rust_web_demo=debug,tower_http=debug")
        .init();

    tracing::info!("启动 Rust Web Demo 应用");
    tracing::info!("配置: {:?}", config);

    // 创建数据库连接池
    let pool = MySqlPool::connect(&config.database.url).await?;
    tracing::info!("数据库连接成功");

    // 运行数据库迁移
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("数据库迁移完成");

    // 初始化服务
    let user_repository = UserRepository::new(pool.clone());
    let auth_service = AuthService::new(
        user_repository.clone(),
        config.jwt.secret,
        config.jwt.expiration,
    );
    let user_service = UserService::new(user_repository.clone(), auth_service.clone());

    let app_state = AppState {
        auth_service,
        user_service,
    };

    // 构建路由
    let app = Router::new()
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
                .layer(cors_layer())
        )
        .with_state(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("服务器启动在 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
