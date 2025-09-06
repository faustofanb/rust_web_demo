use crate::errors::{AppError, AppResult};
use crate::models::{CreateUserRequest, UserResponse};
use crate::repositories::UserRepository;
use crate::services::AuthService;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
    auth_service: AuthService,
}

impl UserService {
    pub fn new(user_repository: UserRepository, auth_service: AuthService) -> Self {
        Self {
            user_repository,
            auth_service,
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> AppResult<UserResponse> {
        // 检查用户名是否已存在
        if self.user_repository.find_by_username(&request.username).await?.is_some() {
            return Err(AppError::Internal(anyhow::anyhow!("用户名已存在")));
        }

        // 检查邮箱是否已存在
        if self.user_repository.find_by_email(&request.email).await?.is_some() {
            return Err(AppError::Internal(anyhow::anyhow!("邮箱已存在")));
        }

        // 哈希密码
        let password_hash = self.auth_service.hash_password(&request.password)?;

        // 创建用户
        let user = self.user_repository
            .create(&request.username, &request.email, &password_hash)
            .await?;

        Ok(UserResponse::from(user))
    }

    pub async fn get_user_by_id(&self, id: i64) -> AppResult<UserResponse> {
        let user = self.user_repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::UserNotFound)?;

        Ok(UserResponse::from(user))
    }

    pub async fn get_user_by_username(&self, username: &str) -> AppResult<UserResponse> {
        let user = self.user_repository
            .find_by_username(username)
            .await?
            .ok_or(AppError::UserNotFound)?;

        Ok(UserResponse::from(user))
    }

    pub async fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repository.list(limit, offset).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn update_user(&self, id: i64, username: Option<String>, email: Option<String>) -> AppResult<UserResponse> {
        let user = self.user_repository
            .update(id, username.as_deref(), email.as_deref())
            .await?;

        Ok(UserResponse::from(user))
    }

    pub async fn delete_user(&self, id: i64) -> AppResult<()> {
        self.user_repository.delete(id).await
    }
}
