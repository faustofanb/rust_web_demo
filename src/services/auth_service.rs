use crate::errors::{AppError, AppResult};
use crate::models::{LoginRequest, LoginResponse, User, UserResponse};
use crate::repositories::UserRepository;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AuthService {
    user_repository: UserRepository,
    jwt_secret: String,
    jwt_expiration: u64,
}

impl AuthService {
    pub fn new(user_repository: UserRepository, jwt_secret: String, jwt_expiration: u64) -> Self {
        Self {
            user_repository,
            jwt_secret,
            jwt_expiration,
        }
    }

    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("密码哈希失败: {}", e)))?;

        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("密码哈希解析失败: {}", e)))?;

        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    pub fn generate_token(&self, user: &User) -> AppResult<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            exp: now + self.jwt_expiration as usize,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims)
    }

    pub async fn register(&self, username: &str, email: &str, password: &str) -> AppResult<UserResponse> {
        // 检查用户名是否已存在
        if self.user_repository.find_by_username(username).await?.is_some() {
            return Err(AppError::Internal(anyhow::anyhow!("用户名已存在")));
        }

        // 检查邮箱是否已存在
        if self.user_repository.find_by_email(email).await?.is_some() {
            return Err(AppError::Internal(anyhow::anyhow!("邮箱已存在")));
        }

        // 哈希密码
        let password_hash = self.hash_password(password)?;

        // 创建用户
        let user = self.user_repository.create(username, email, &password_hash).await?;

        Ok(UserResponse::from(user))
    }

    pub async fn login(&self, request: LoginRequest) -> AppResult<LoginResponse> {
        // 查找用户
        let user = self.user_repository
            .find_by_username(&request.username)
            .await?
            .ok_or(AppError::AuthenticationFailed)?;

        // 验证密码
        if !self.verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::AuthenticationFailed);
        }

        // 生成token
        let token = self.generate_token(&user)?;

        Ok(LoginResponse {
            user: UserResponse::from(user),
            token,
        })
    }

    pub async fn get_user_from_token(&self, token: &str) -> AppResult<UserResponse> {
        let claims = self.verify_token(token)?;
        
        let user = self.user_repository
            .find_by_id(claims.sub.parse::<i64>().unwrap())
            .await?
            .ok_or(AppError::AuthenticationFailed)?;

        Ok(UserResponse::from(user))
    }
}
