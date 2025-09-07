use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::config::AppConfig;
use crate::error::AppError;

/// 请求验证中间件
pub async fn validation_middleware(
    State(_config): State<Arc<AppConfig>>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 这里可以添加通用的请求验证逻辑
    // 例如：检查请求大小、验证必要的头部等

    // 检查Content-Type（对于POST/PUT请求）
    if matches!(request.method(), &axum::http::Method::POST | &axum::http::Method::PUT) {
        if let Some(content_type) = request.headers().get("content-type") {
            if !content_type.to_str().unwrap_or("").starts_with("application/json") {
                return Err(AppError::DomainError("Content-Type must be application/json".to_string()));
            }
        }
    }

    // 检查请求大小（防止DoS攻击）
    if let Some(content_length) = request.headers().get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024; // 10MB
                if length > MAX_REQUEST_SIZE {
                    return Err(AppError::DomainError("Request too large".to_string()));
                }
            }
        }
    }

    Ok(next.run(request).await)
}

/// 租户验证中间件
pub async fn tenant_validation_middleware(
    State(_config): State<Arc<AppConfig>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 从请求头中提取租户信息
    let tenant_id = extract_tenant_from_request(&request)?;
    
    // 将租户ID添加到请求扩展中
    request.extensions_mut().insert(tenant_id);

    Ok(next.run(request).await)
}

/// 从请求中提取租户ID
fn extract_tenant_from_request(request: &Request) -> Result<uuid::Uuid, AppError> {
    // 方法1: 从X-Tenant-ID头部获取
    if let Some(tenant_header) = request.headers().get("X-Tenant-ID") {
        if let Ok(tenant_str) = tenant_header.to_str() {
            if let Ok(tenant_id) = uuid::Uuid::parse_str(tenant_str) {
                return Ok(tenant_id);
            }
        }
    }

    // 方法2: 从子域名获取（如果使用子域名多租户）
    if let Some(host) = request.headers().get("host") {
        if let Ok(host_str) = host.to_str() {
            if let Some(subdomain) = extract_subdomain(host_str) {
                // 这里可以将子域名映射到租户ID
                // 在实际应用中，你可能需要查询数据库来获取租户ID
                if let Ok(tenant_id) = uuid::Uuid::parse_str(&subdomain) {
                    return Ok(tenant_id);
                }
            }
        }
    }

    // 方法3: 从JWT token中获取（如果用户已认证）
    if let Some(auth_header) = request.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let _token = &auth_str[7..];
                // 这里可以解析JWT token获取租户ID
                // 为了简化，我们返回一个默认的租户ID
                return Ok(uuid::Uuid::new_v4());
            }
        }
    }

    // 如果没有找到租户信息，返回错误
    Err(AppError::AuthenticationError("Tenant information is required".to_string()))
}

/// 从主机名中提取子域名
fn extract_subdomain(host: &str) -> Option<String> {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() > 2 {
        Some(parts[0].to_string())
    } else {
        None
    }
}

/// 从请求扩展中获取租户ID
pub fn extract_tenant_from_extensions(request: &Request) -> Result<uuid::Uuid, AppError> {
    request
        .extensions()
        .get::<uuid::Uuid>()
        .copied()
        .ok_or_else(|| AppError::AuthenticationError("Tenant information not found".to_string()))
}

/// 请求日志中间件
pub async fn request_logging_middleware(
    State(_config): State<Arc<AppConfig>>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let start = std::time::Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    tracing::info!(
        "Request started: {} {} (User-Agent: {})",
        method,
        uri,
        user_agent
    );

    let response = next.run(request).await;

    let duration = start.elapsed();
    tracing::info!(
        "Request completed: {} {} in {:?}",
        method,
        uri,
        duration
    );

    Ok(response)
}
