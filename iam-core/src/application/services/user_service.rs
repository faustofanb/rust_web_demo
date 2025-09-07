use std::sync::Arc;
use uuid::Uuid;
use crate::domain::identity_access::aggregates::user::User;
use crate::domain::identity_access::commands::{RegisterUserCommand, UpdateUserCommand, DeactivateUserCommand};
use crate::domain::identity_access::events::IdentityAccessEvent;
use crate::infrastructure::persistence::event_store::EventStore;
use crate::error::AppError;
use anyhow::Result;

pub struct UserService {
    event_store: Arc<dyn EventStore>,
}

impl UserService {
    pub fn new(event_store: Arc<dyn EventStore>) -> Self {
        Self { event_store }
    }

    pub async fn register_user(&self, command: RegisterUserCommand) -> Result<Uuid, AppError> {
        let user_id = Uuid::new_v4();

        // 1. Execute business logic on the aggregate.
        // Since this is a new user, we don't need to load any previous events.
        // The aggregate's `register` function returns an event.
        let event = User::register(
            user_id,
            command.tenant_id,
            command.username,
            command.email,
            command.password_hash,
        )
        .map_err(|e| AppError::DomainError(e.to_string()))?;

        // 2. Save the new event to the event store.
        // For a new aggregate, the expected version is 0.
        self.event_store.save_events(user_id, &[event], 0).await?;

        Ok(user_id)
    }

    pub async fn update_user(&self, command: UpdateUserCommand) -> Result<(), AppError> {
        // 1. Load existing events to reconstruct the aggregate
        let stored_events = self.event_store.load_events(command.user_id).await?;
        let events: Vec<IdentityAccessEvent> = stored_events
            .iter()
            .map(|stored_event| serde_json::from_value(stored_event.payload.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::SerializationError(e))?;
        
        let user = User::from_events(&events);

        // 2. Execute business logic on the aggregate
        let event = user.update(command.username, command.email)
            .map_err(|e| AppError::DomainError(e.to_string()))?;

        // 3. Save the new event to the event store
        self.event_store.save_events(command.user_id, &[event], user.version()).await?;

        Ok(())
    }

    pub async fn deactivate_user(&self, command: DeactivateUserCommand) -> Result<(), AppError> {
        // 1. Load existing events to reconstruct the aggregate
        let stored_events = self.event_store.load_events(command.user_id).await?;
        let events: Vec<IdentityAccessEvent> = stored_events
            .iter()
            .map(|stored_event| serde_json::from_value(stored_event.payload.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::SerializationError(e))?;
        
        let user = User::from_events(&events);

        // 2. Execute business logic on the aggregate
        let event = user.deactivate(command.reason)
            .map_err(|e| AppError::DomainError(e.to_string()))?;

        // 3. Save the new event to the event store
        self.event_store.save_events(command.user_id, &[event], user.version()).await?;

        Ok(())
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        let stored_events = self.event_store.load_events(user_id).await?;
        let events: Vec<IdentityAccessEvent> = stored_events
            .iter()
            .map(|stored_event| serde_json::from_value(stored_event.payload.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::SerializationError(e))?;
        
        Ok(User::from_events(&events))
    }
}
