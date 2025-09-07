use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::config::AppConfig;

/// 应用程序指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetrics {
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub active_connections: u32,
    pub memory_usage_mb: f64,
}

/// 指标收集器
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    start_time: Instant,
    total_requests: Arc<RwLock<u64>>,
    successful_requests: Arc<RwLock<u64>>,
    failed_requests: Arc<RwLock<u64>>,
    response_times: Arc<RwLock<Vec<Duration>>>,
    active_connections: Arc<RwLock<u32>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_requests: Arc::new(RwLock::new(0)),
            successful_requests: Arc::new(RwLock::new(0)),
            failed_requests: Arc::new(RwLock::new(0)),
            response_times: Arc::new(RwLock::new(Vec::new())),
            active_connections: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn record_request(&self, success: bool, response_time: Duration) {
        // 记录总请求数
        {
            let mut total = self.total_requests.write().await;
            *total += 1;
        }

        // 记录成功/失败请求数
        if success {
            let mut successful = self.successful_requests.write().await;
            *successful += 1;
        } else {
            let mut failed = self.failed_requests.write().await;
            *failed += 1;
        }

        // 记录响应时间
        {
            let mut times = self.response_times.write().await;
            times.push(response_time);
            
            // 只保留最近1000个响应时间
            if times.len() > 1000 {
                times.remove(0);
            }
        }
    }

    pub async fn increment_active_connections(&self) {
        let mut active = self.active_connections.write().await;
        *active += 1;
    }

    pub async fn decrement_active_connections(&self) {
        let mut active = self.active_connections.write().await;
        if *active > 0 {
            *active -= 1;
        }
    }

    pub async fn get_metrics(&self) -> AppMetrics {
        let uptime = self.start_time.elapsed();
        let total_requests = *self.total_requests.read().await;
        let successful_requests = *self.successful_requests.read().await;
        let failed_requests = *self.failed_requests.read().await;
        let active_connections = *self.active_connections.read().await;

        // 计算平均响应时间
        let response_times = self.response_times.read().await;
        let average_response_time = if response_times.is_empty() {
            0.0
        } else {
            let total_time: Duration = response_times.iter().sum();
            total_time.as_millis() as f64 / response_times.len() as f64
        };

        // 获取内存使用情况（简化版本）
        let memory_usage = get_memory_usage();

        AppMetrics {
            uptime_seconds: uptime.as_secs(),
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time_ms: average_response_time,
            active_connections,
            memory_usage_mb: memory_usage,
        }
    }
}

/// 获取内存使用情况（MB）
fn get_memory_usage() -> f64 {
    // 这是一个简化的实现，在实际应用中可能需要更复杂的逻辑
    // 或者使用专门的监控库
    0.0
}

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub environment: String,
    pub database_status: String,
    pub metrics: AppMetrics,
}

/// 健康检查端点
pub async fn health_check(
    State(config): State<Arc<AppConfig>>,
    State(metrics): State<Arc<MetricsCollector>>,
) -> Result<Json<HealthCheckResponse>, StatusCode> {
    let metrics_data = metrics.get_metrics().await;
    
    // 检查数据库连接状态
    let database_status = check_database_health(&config).await;

    let response = HealthCheckResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: config.environment.clone(),
        database_status,
        metrics: metrics_data,
    };

    Ok(Json(response))
}

/// 指标端点
pub async fn metrics_endpoint(
    State(metrics): State<Arc<MetricsCollector>>,
) -> Result<Json<AppMetrics>, StatusCode> {
    let metrics_data = metrics.get_metrics().await;
    Ok(Json(metrics_data))
}

/// 检查数据库健康状态
async fn check_database_health(config: &AppConfig) -> String {
    match sqlx::MySqlPool::connect(&config.database.url).await {
        Ok(pool) => {
            match sqlx::query("SELECT 1").fetch_one(&pool).await {
                Ok(_) => "connected".to_string(),
                Err(_) => "disconnected".to_string(),
            }
        }
        Err(_) => "disconnected".to_string(),
    }
}

/// 请求指标中间件
pub async fn metrics_middleware(
    State(metrics): State<Arc<MetricsCollector>>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let start = std::time::Instant::now();
    
    // 增加活跃连接数
    metrics.increment_active_connections().await;
    
    let response = next.run(request).await;
    let response_time = start.elapsed();
    
    // 减少活跃连接数
    metrics.decrement_active_connections().await;
    
    // 记录请求指标
    let success = response.status().is_success();
    metrics.record_request(success, response_time).await;
    
    response
}
