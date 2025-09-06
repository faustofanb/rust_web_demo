use axum::{
    extract::{Path, Query, State},
    http::{Method, Uri},
    response::Response,
    routing::any,
    Router,
};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use std::time::Duration;

use shared::{
    config::MicroserviceConfig,
    registry::{ServiceRegistry, ServiceInstance},
    tracing::init_tracing,
};

#[derive(Clone)]
pub struct GatewayState {
    pub service_registry: ServiceRegistry,
    pub http_client: Client,
    pub routes: HashMap<String, String>, // 路径模式 -> 服务名
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = MicroserviceConfig::from_env()?;
    
    // 初始化日志
    init_tracing(&config.service.name)?;

    tracing::info!("启动 {} 网关", config.service.name);

    // 创建服务注册器
    let service_registry = ServiceRegistry::new(
        &config.consul,
        config.service.name.clone(),
        format!("{}-{}", config.service.name, uuid::Uuid::new_v4()),
        config.service.host.clone(),
        config.service.port,
    )?;

    // 创建HTTP客户端
    let http_client = Client::new();

    // 配置路由规则
    let mut routes = HashMap::new();
    routes.insert("/api/users".to_string(), "user-service".to_string());
    routes.insert("/api/orders".to_string(), "order-service".to_string());
    routes.insert("/api/products".to_string(), "product-service".to_string());

    let gateway_state = GatewayState {
        service_registry,
        http_client,
        routes,
    };

    // 构建路由
    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/*path", any(proxy_request))
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(CorsLayer::permissive())
                .layer(TraceLayer::new_for_http())
        )
        .with_state(gateway_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("网关启动在 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(json!({
        "status": "healthy",
        "service": "gateway",
        "timestamp": chrono::Utc::now()
    }))
}

async fn proxy_request(
    State(state): State<GatewayState>,
    method: Method,
    uri: Uri,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<Response<axum::body::Body>, axum::http::StatusCode> {
    let path = uri.path();
    
    // 1. 路由匹配
    let service_name = match match_route(&state.routes, path) {
        Some(service) => service,
        None => {
            tracing::warn!("未找到匹配的路由: {}", path);
            return Ok(Response::builder()
                .status(axum::http::StatusCode::NOT_FOUND)
                .body(axum::body::Body::from(json!({
                    "error": "路由未找到",
                    "path": path
                }).to_string()))
                .unwrap());
        }
    };

    // 2. 服务发现
    let service_instance = match state.service_registry.select_instance(&service_name).await {
        Ok(Some(instance)) => instance,
        Ok(None) => {
            tracing::error!("服务 {} 没有可用实例", service_name);
            return Ok(Response::builder()
                .status(axum::http::StatusCode::SERVICE_UNAVAILABLE)
                .body(axum::body::Body::from(json!({
                    "error": "服务不可用",
                    "service": service_name
                }).to_string()))
                .unwrap());
        }
        Err(e) => {
            tracing::error!("服务发现失败: {}", e);
            return Ok(Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from(json!({
                    "error": "服务发现失败"
                }).to_string()))
                .unwrap());
        }
    };

    // 3. 构建目标URL
    let target_url = format!("http://{}:{}{}", 
        service_instance.address, 
        service_instance.port, 
        path
    );

    tracing::info!("代理请求: {} {} -> {}", method, path, target_url);

    // 4. 转发请求
    let mut request_builder = state.http_client
        .request(method, &target_url)
        .body(body.to_vec());

    // 复制必要的请求头
    for (key, value) in headers.iter() {
        if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(key.as_str().as_bytes()) {
            if let Ok(header_value) = reqwest::header::HeaderValue::from_bytes(value.as_bytes()) {
                request_builder = request_builder.header(header_name, header_value);
            }
        }
    }

    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            let response_headers = response.headers().clone();
            let response_body = response.bytes().await.unwrap_or_default();

            let mut axum_response = Response::builder().status(status);
            
            // 复制响应头
            for (key, value) in response_headers.iter() {
                if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_str().as_bytes()) {
                    if let Ok(header_value) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
                        axum_response = axum_response.header(header_name, header_value);
                    }
                }
            }

            axum_response
                .body(axum::body::Body::from(response_body))
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            tracing::error!("代理请求失败: {}", e);
            Ok(Response::builder()
                .status(axum::http::StatusCode::BAD_GATEWAY)
                .body(axum::body::Body::from(json!({
                    "error": "代理请求失败",
                    "message": e.to_string()
                }).to_string()))
                .unwrap())
        }
    }
}

fn match_route(routes: &HashMap<String, String>, path: &str) -> Option<String> {
    for (pattern, service) in routes {
        if path.starts_with(pattern) {
            return Some(service.clone());
        }
    }
    None
}
