use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub tenant_id: String,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
    pub tenant_id: Uuid,
}

/// JWT认证中间件
pub async fn auth_middleware(
    State(config): State<Arc<AppConfig>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 从请求头中获取Authorization token
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::AuthenticationError("Missing authorization header".to_string()))?;

    // 检查Bearer token格式
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthenticationError("Invalid authorization header format".to_string()));
    }

    let token = &auth_header[7..]; // 移除"Bearer "前缀

    // 验证JWT token
    let claims = validate_token(token, &config.jwt.secret)?;

    // 将用户信息添加到请求扩展中
    let authenticated_user = AuthenticatedUser {
        user_id: Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::AuthenticationError("Invalid user ID in token".to_string()))?,
        username: claims.username,
        tenant_id: Uuid::parse_str(&claims.tenant_id)
            .map_err(|_| AppError::AuthenticationError("Invalid tenant ID in token".to_string()))?,
    };

    request.extensions_mut().insert(authenticated_user);

    Ok(next.run(request).await)
}

/// 验证JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?;

    // 检查token是否过期
    let now = chrono::Utc::now().timestamp() as u64;
    if token_data.claims.exp < now {
        return Err(AppError::AuthenticationError("Token has expired".to_string()));
    }

    Ok(token_data.claims)
}

/// 生成JWT token
pub fn generate_token(
    user_id: Uuid,
    username: String,
    tenant_id: Uuid,
    secret: &str,
    expiration_hours: u64,
) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as u64;
    let exp = now + (expiration_hours * 3600);

    let claims = Claims {
        sub: user_id.to_string(),
        username,
        tenant_id: tenant_id.to_string(),
        exp,
        iat: now,
    };

    let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_ref());
    let header = jsonwebtoken::Header::new(Algorithm::HS256);

    jsonwebtoken::encode(&header, &claims, &encoding_key)
        .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))
}

/// 从请求中提取认证用户信息
pub fn extract_authenticated_user(request: &Request) -> Result<&AuthenticatedUser, AppError> {
    request
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or_else(|| AppError::AuthenticationError("User not authenticated".to_string()))
}
