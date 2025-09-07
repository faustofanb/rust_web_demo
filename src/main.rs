// 使用lib.rs中定义的模块
use rust_web_demo::{create_router, init_tracing, AppConfig, AppState};

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    init_tracing();

    // 加载配置
    let config = AppConfig::from_env()?;
    tracing::info!("配置: {:?}", config);

    // 初始化应用状态
    let app_state = AppState::new(&config).await?;
    tracing::info!("应用状态初始化完成");

    // 构建路由
    let app = create_router(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("服务器启动在 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
