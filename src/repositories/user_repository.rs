use crate::errors::{AppError, AppResult};
use crate::models::User;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, username: &str, email: &str, password_hash: &str) -> AppResult<User> {
        let result = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES (?, ?, ?)
            "#,
            username,
            email,
            password_hash
        )
        .execute(&self.pool)
        .await?;

        let user = self.find_by_id(result.last_insert_id() as i64).await?
            .ok_or(AppError::Internal(anyhow::anyhow!("创建用户后无法找到用户")))?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: i64) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, id: i64, username: Option<&str>, email: Option<&str>) -> AppResult<User> {
        if let Some(username) = username {
            sqlx::query!(
                r#"
                UPDATE users SET username = ?, updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                "#,
                username,
                id
            )
            .execute(&self.pool)
            .await?;
        }

        if let Some(email) = email {
            sqlx::query!(
                r#"
                UPDATE users SET email = ?, updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                "#,
                email,
                id
            )
            .execute(&self.pool)
            .await?;
        }

        // 返回更新后的用户
        self.find_by_id(id)
            .await?
            .ok_or(AppError::UserNotFound)
    }

    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::UserNotFound);
        }

        Ok(())
    }

    pub async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<User>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}
