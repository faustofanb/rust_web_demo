use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", service_name).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("{} 日志系统初始化完成", service_name);
    Ok(())
}
