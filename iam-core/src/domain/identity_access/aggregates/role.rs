use uuid::Uuid;
use crate::domain::identity_access::events::{
    IdentityAccessEvent, RoleCreated, RoleUpdated, RoleDeleted
};
use anyhow::{Result, anyhow};

/// The state of the Role aggregate.
#[derive(Debug, Default)]
pub struct Role {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    code: String,
    description: Option<String>,
    version: u64,
}

impl Role {
    /// Business logic for creating a new role.
    pub fn create(
        id: Uuid,
        tenant_id: Uuid,
        name: String,
        code: String,
        description: Option<String>,
    ) -> Result<IdentityAccessEvent> {
        // Basic validation
        if name.is_empty() {
            return Err(anyhow!("Role name cannot be empty"));
        }
        if code.is_empty() {
            return Err(anyhow!("Role code cannot be empty"));
        }

        // Validate code format (alphanumeric and underscores only)
        if !code.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(anyhow!("Role code can only contain alphanumeric characters and underscores"));
        }

        Ok(IdentityAccessEvent::RoleCreated(RoleCreated {
            role_id: id,
            tenant_id,
            name,
            code,
            description,
        }))
    }

    /// Business logic for updating a role.
    pub fn update(&self, name: Option<String>, description: Option<String>) -> Result<IdentityAccessEvent> {
        if let Some(ref name) = name {
            if name.is_empty() {
                return Err(anyhow!("Role name cannot be empty"));
            }
        }

        Ok(IdentityAccessEvent::RoleUpdated(RoleUpdated {
            role_id: self.id,
            name,
            description,
        }))
    }

    /// Business logic for deleting a role.
    pub fn delete(&self) -> Result<IdentityAccessEvent> {
        Ok(IdentityAccessEvent::RoleDeleted(RoleDeleted {
            role_id: self.id,
        }))
    }

    /// Applies an event to the aggregate to change its state.
    pub fn apply(&mut self, event: &IdentityAccessEvent) {
        match event {
            IdentityAccessEvent::RoleCreated(e) => {
                self.id = e.role_id;
                self.tenant_id = e.tenant_id;
                self.name = e.name.clone();
                self.code = e.code.clone();
                self.description = e.description.clone();
            }
            IdentityAccessEvent::RoleUpdated(e) => {
                if let Some(ref name) = e.name {
                    self.name = name.clone();
                }
                if let Some(ref description) = e.description {
                    self.description = Some(description.clone());
                }
            }
            IdentityAccessEvent::RoleDeleted(_) => {
                // Role is deleted, but we keep the state for audit purposes
                // In a real system, you might want to mark it as deleted
            }
            _ => {
                // Other events don't affect role state
            }
        }
        self.version += 1;
    }

    /// Reconstructs the aggregate state from a series of events.
    pub fn from_events(events: &[IdentityAccessEvent]) -> Self {
        let mut role = Role::default();
        for event in events {
            role.apply(event);
        }
        role
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}
