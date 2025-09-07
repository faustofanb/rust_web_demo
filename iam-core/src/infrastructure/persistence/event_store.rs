use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::identity_access::events::IdentityAccessEvent;
use crate::error::AppError;

/// Trait for an event store.
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Saves events to the store for a given aggregate.
    /// This operation must be atomic.
    async fn save_events(&self, aggregate_id: Uuid, events: &[IdentityAccessEvent], expected_version: u64) -> Result<(), AppError>;

    /// Loads all events for a given aggregate.
    async fn load_events(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, AppError>;
}

/// Represents an event as it is stored in the database.
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct StoredEvent {
    pub id: Uuid,
    pub aggregate_id: Uuid,
    pub sequence: u64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// We can now implement the `EventStore` trait for `SqlxEventStore` or a `InMemoryEventStore` for tests.

/*
-- SQL for creating the events table in MySQL:
CREATE TABLE events (
    id BINARY(16) PRIMARY KEY,
    aggregate_id BINARY(16) NOT NULL,
    sequence BIGINT UNSIGNED NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    payload JSON NOT NULL,
    created_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    UNIQUE KEY uq_aggregate_sequence (aggregate_id, sequence)
);
*/

use sqlx::{MySqlPool, Row};

pub struct SqlxEventStore {
    pool: MySqlPool,
}

impl SqlxEventStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventStore for SqlxEventStore {
    async fn save_events(&self, aggregate_id: Uuid, events: &[IdentityAccessEvent], expected_version: u64) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        let current_version: Option<i64> = sqlx::query("SELECT MAX(sequence) FROM events WHERE aggregate_id = ?")
            .bind(aggregate_id)
            .fetch_optional(&mut *tx)
            .await?
            .map(|row| row.get(0))
            .flatten();

        let current_version = current_version.unwrap_or(0) as u64;

        if current_version != expected_version {
            return Err(AppError::ConcurrencyConflict);
        }

        let mut sequence = current_version;
        for event in events {
            sequence += 1;
            let payload = serde_json::to_value(event)?;
            let event_type = match event {
                IdentityAccessEvent::UserRegistered(_) => "UserRegistered",
                IdentityAccessEvent::UserUpdated(_) => "UserUpdated",
                IdentityAccessEvent::UserDeactivated(_) => "UserDeactivated",
                IdentityAccessEvent::RoleCreated(_) => "RoleCreated",
                IdentityAccessEvent::RoleUpdated(_) => "RoleUpdated",
                IdentityAccessEvent::RoleDeleted(_) => "RoleDeleted",
                IdentityAccessEvent::PermissionCreated(_) => "PermissionCreated",
                IdentityAccessEvent::UserRoleAssigned(_) => "UserRoleAssigned",
                IdentityAccessEvent::UserRoleRemoved(_) => "UserRoleRemoved",
                IdentityAccessEvent::RolePermissionGranted(_) => "RolePermissionGranted",
                IdentityAccessEvent::RolePermissionRevoked(_) => "RolePermissionRevoked",
            };

            sqlx::query(
                "INSERT INTO events (id, aggregate_id, sequence, event_type, payload) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(Uuid::new_v4())
            .bind(aggregate_id)
            .bind(sequence)
            .bind(event_type)
            .bind(payload)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn load_events(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, AppError> {
        let stored_events = sqlx::query_as::<_, StoredEvent>(
            "SELECT id, aggregate_id, sequence, event_type, payload, created_at FROM events WHERE aggregate_id = ? ORDER BY sequence ASC"
        )
        .bind(aggregate_id)
        .fetch_all(&self.pool)
        .await?;

        if stored_events.is_empty() {
            return Err(AppError::AggregateNotFound(aggregate_id.to_string()));
        }

        Ok(stored_events)
    }
}