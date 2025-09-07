use crate::application::dtos as user_view;
use crate::domain::identity_access::events::IdentityAccessEvent;
use crate::infrastructure::persistence::event_store::StoredEvent;
use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, EntityTrait};

pub struct UserProjector {
    db: DatabaseConnection,
}

impl UserProjector {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn handle_event(&self, event: &StoredEvent) -> Result<()> {
        match event.event_type.as_str() {
            "UserRegistered" => {
                let payload: IdentityAccessEvent = serde_json::from_value(event.payload.clone())?;
                let user_registered = match payload {
                    IdentityAccessEvent::UserRegistered(user_registered) => user_registered,
                    _ => return Err(anyhow::anyhow!("Invalid event type")),
                };

                let new_user = user_view::ActiveModel {
                    id: Set(user_registered.user_id.into()),
                    tenant_id: Set(user_registered.tenant_id.into()),
                    username: Set(user_registered.username),
                    email: Set(user_registered.email),
                    password_hash: Set(user_registered.password_hash),
                    status: Set("active".to_string()),
                    created_at: Set(event.created_at.into()),
                    updated_at: Set(event.created_at.into()),
                };

                new_user.insert(&self.db).await?;
            }
            "UserUpdated" => {
                let payload: IdentityAccessEvent = serde_json::from_value(event.payload.clone())?;
                let user_updated = match payload {
                    IdentityAccessEvent::UserUpdated(user_updated) => user_updated,
                    _ => return Err(anyhow::anyhow!("Invalid event type")),
                };

                let mut user: user_view::ActiveModel = user_view::Entity::find_by_id(user_updated.user_id)
                    .one(&self.db)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("User not found"))?
                    .into();

                if let Some(username) = user_updated.username {
                    user.username = Set(username);
                }
                if let Some(email) = user_updated.email {
                    user.email = Set(email);
                }
                user.updated_at = Set(event.created_at.into());

                user.update(&self.db).await?;
            }
            "UserDeactivated" => {
                let payload: IdentityAccessEvent = serde_json::from_value(event.payload.clone())?;
                let user_deactivated = match payload {
                    IdentityAccessEvent::UserDeactivated(user_deactivated) => user_deactivated,
                    _ => return Err(anyhow::anyhow!("Invalid event type")),
                };

                let mut user: user_view::ActiveModel = user_view::Entity::find_by_id(user_deactivated.user_id)
                    .one(&self.db)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("User not found"))?
                    .into();

                user.status = Set("inactive".to_string());
                user.updated_at = Set(event.created_at.into());

                user.update(&self.db).await?;
            }
            // Other event types can be handled here...
            _ => {}
        }
        Ok(())
    }
}