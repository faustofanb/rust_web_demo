use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/*
-- SQL for creating the users_view table in MySQL:
CREATE TABLE users_view (
    id BINARY(16) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NOT NULL
);
*/

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users_view")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub status: String,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Re-export uuid and chrono types for convenience
pub use uuid::Uuid;
pub use chrono::{DateTime as ChronoDateTime, Utc as ChronoUtc};
type ChronoDateTimeUtc = ChronoDateTime<ChronoUtc>;
