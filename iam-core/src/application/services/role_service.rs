use std::sync::Arc;
use uuid::Uuid;
use crate::domain::identity_access::aggregates::role::Role;
use crate::domain::identity_access::commands::{
    CreateRoleCommand, UpdateRoleCommand, DeleteRoleCommand,
    AssignUserRoleCommand, RemoveUserRoleCommand
};
use crate::domain::identity_access::events::IdentityAccessEvent;
use crate::infrastructure::persistence::event_store::EventStore;
use crate::error::AppError;
use anyhow::Result;

pub struct RoleService {
    event_store: Arc<dyn EventStore>,
}

impl RoleService {
    pub fn new(event_store: Arc<dyn EventStore>) -> Self {
        Self { event_store }
    }

    pub async fn create_role(&self, command: CreateRoleCommand) -> Result<Uuid, AppError> {
        let role_id = Uuid::new_v4();

        // 1. Execute business logic on the aggregate.
        let event = Role::create(
            role_id,
            command.tenant_id,
            command.name,
            command.code,
            command.description,
        )
        .map_err(|e| AppError::DomainError(e.to_string()))?;

        // 2. Save the new event to the event store.
        self.event_store.save_events(role_id, &[event], 0).await?;

        Ok(role_id)
    }

    pub async fn update_role(&self, command: UpdateRoleCommand) -> Result<(), AppError> {
        // 1. Load existing events to reconstruct the aggregate
        let stored_events = self.event_store.load_events(command.role_id).await?;
        let events: Vec<IdentityAccessEvent> = stored_events
            .iter()
            .map(|stored_event| serde_json::from_value(stored_event.payload.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::SerializationError(e))?;
        
        let role = Role::from_events(&events);

        // 2. Execute business logic on the aggregate
        let event = role.update(command.name, command.description)
            .map_err(|e| AppError::DomainError(e.to_string()))?;

        // 3. Save the new event to the event store
        self.event_store.save_events(command.role_id, &[event], role.version()).await?;

        Ok(())
    }

    pub async fn delete_role(&self, command: DeleteRoleCommand) -> Result<(), AppError> {
        // 1. Load existing events to reconstruct the aggregate
        let stored_events = self.event_store.load_events(command.role_id).await?;
        let events: Vec<IdentityAccessEvent> = stored_events
            .iter()
            .map(|stored_event| serde_json::from_value(stored_event.payload.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::SerializationError(e))?;
        
        let role = Role::from_events(&events);

        // 2. Execute business logic on the aggregate
        let event = role.delete()
            .map_err(|e| AppError::DomainError(e.to_string()))?;

        // 3. Save the new event to the event store
        self.event_store.save_events(command.role_id, &[event], role.version()).await?;

        Ok(())
    }

    pub async fn assign_user_role(&self, command: AssignUserRoleCommand) -> Result<(), AppError> {
        let event = IdentityAccessEvent::UserRoleAssigned(crate::domain::identity_access::events::UserRoleAssigned {
            user_id: command.user_id,
            role_id: command.role_id,
        });

        // Save the event to the event store
        // Note: In a real system, you might want to save this to a separate aggregate
        // or use a different approach for user-role relationships
        self.event_store.save_events(command.user_id, &[event], 0).await?;

        Ok(())
    }

    pub async fn remove_user_role(&self, command: RemoveUserRoleCommand) -> Result<(), AppError> {
        let event = IdentityAccessEvent::UserRoleRemoved(crate::domain::identity_access::events::UserRoleRemoved {
            user_id: command.user_id,
            role_id: command.role_id,
        });

        // Save the event to the event store
        self.event_store.save_events(command.user_id, &[event], 0).await?;

        Ok(())
    }

    pub async fn get_role(&self, role_id: Uuid) -> Result<Role, AppError> {
        let stored_events = self.event_store.load_events(role_id).await?;
        let events: Vec<IdentityAccessEvent> = stored_events
            .iter()
            .map(|stored_event| serde_json::from_value(stored_event.payload.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::SerializationError(e))?;
        
        Ok(Role::from_events(&events))
    }
}
