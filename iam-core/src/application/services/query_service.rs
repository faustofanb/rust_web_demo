use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QuerySelect};
use uuid::Uuid;
use crate::application::dtos as user_view;
use crate::error::AppError;

pub struct QueryService {
    db: DatabaseConnection,
}

impl QueryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据ID查询用户
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<user_view::Model>, AppError> {
        let user = user_view::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(user)
    }

    /// 根据用户名查询用户
    pub async fn get_user_by_username(&self, username: &str, tenant_id: Uuid) -> Result<Option<user_view::Model>, AppError> {
        let user = user_view::Entity::find()
            .filter(user_view::Column::Username.eq(username))
            .filter(user_view::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(user)
    }

    /// 根据邮箱查询用户
    pub async fn get_user_by_email(&self, email: &str, tenant_id: Uuid) -> Result<Option<user_view::Model>, AppError> {
        let user = user_view::Entity::find()
            .filter(user_view::Column::Email.eq(email))
            .filter(user_view::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(user)
    }

    /// 获取租户下的所有用户
    pub async fn get_users_by_tenant(&self, tenant_id: Uuid, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<user_view::Model>, AppError> {
        let mut query = user_view::Entity::find()
            .filter(user_view::Column::TenantId.eq(tenant_id));

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        if let Some(offset) = offset {
            query = query.offset(offset);
        }

        let users = query
            .all(&self.db)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(users)
    }

    /// 根据状态查询用户
    pub async fn get_users_by_status(&self, status: &str, tenant_id: Uuid) -> Result<Vec<user_view::Model>, AppError> {
        let users = user_view::Entity::find()
            .filter(user_view::Column::Status.eq(status))
            .filter(user_view::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(users)
    }

    /// 验证用户凭据
    pub async fn validate_user_credentials(&self, username: &str, password: &str, tenant_id: Uuid) -> Result<Option<user_view::Model>, AppError> {
        let user = self.get_user_by_username(username, tenant_id).await?;

        if let Some(user) = user {
            let password_valid = bcrypt::verify(password, &user.password_hash)
                .map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))?;

            if password_valid && user.status == "active" {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }
}
