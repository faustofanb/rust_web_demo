use uuid::Uuid;
use crate::domain::identity_access::events::{
    IdentityAccessEvent, UserRegistered, UserUpdated, UserDeactivated
};
use anyhow::{Result, anyhow};

/// The state of the User aggregate.
#[derive(Debug, Default)]
pub struct User {
    id: Uuid,
    tenant_id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    status: UserStatus,
    version: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Locked,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Active
    }
}

impl User {
    /// Business logic for registering a new user.
    /// This function validates inputs and, if successful, returns a `UserRegistered` event.
    pub fn register(
        id: Uuid,
        tenant_id: Uuid,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<IdentityAccessEvent> {
        // Basic validation
        if username.is_empty() {
            return Err(anyhow!("Username cannot be empty"));
        }
        if email.is_empty() || !email.contains('@') {
            return Err(anyhow!("Invalid email format"));
        }
        if password_hash.is_empty() {
            return Err(anyhow!("Password hash cannot be empty"));
        }

        // If validation passes, create the event.
        // The aggregate state is NOT changed here directly.
        Ok(IdentityAccessEvent::UserRegistered(UserRegistered {
            user_id: id,
            tenant_id,
            username,
            email,
            password_hash,
        }))
    }

    /// Business logic for updating a user.
    pub fn update(&self, username: Option<String>, email: Option<String>) -> Result<IdentityAccessEvent> {
        if self.status != UserStatus::Active {
            return Err(anyhow!("Cannot update inactive or locked user"));
        }

        if let Some(ref username) = username {
            if username.is_empty() {
                return Err(anyhow!("Username cannot be empty"));
            }
        }

        if let Some(ref email) = email {
            if email.is_empty() || !email.contains('@') {
                return Err(anyhow!("Invalid email format"));
            }
        }

        Ok(IdentityAccessEvent::UserUpdated(UserUpdated {
            user_id: self.id,
            username,
            email,
        }))
    }

    /// Business logic for deactivating a user.
    pub fn deactivate(&self, reason: String) -> Result<IdentityAccessEvent> {
        if self.status != UserStatus::Active {
            return Err(anyhow!("User is already inactive or locked"));
        }

        Ok(IdentityAccessEvent::UserDeactivated(UserDeactivated {
            user_id: self.id,
            reason,
        }))
    }

    /// Applies an event to the aggregate to change its state.
    /// This is how we reconstruct the state from an event stream.
    pub fn apply(&mut self, event: &IdentityAccessEvent) {
        match event {
            IdentityAccessEvent::UserRegistered(e) => {
                self.id = e.user_id;
                self.tenant_id = e.tenant_id;
                self.username = e.username.clone();
                self.email = e.email.clone();
                self.password_hash = e.password_hash.clone();
                self.status = UserStatus::Active;
            }
            IdentityAccessEvent::UserUpdated(e) => {
                if let Some(ref username) = e.username {
                    self.username = username.clone();
                }
                if let Some(ref email) = e.email {
                    self.email = email.clone();
                }
            }
            IdentityAccessEvent::UserDeactivated(_) => {
                self.status = UserStatus::Inactive;
            }
            _ => {
                // Other events don't affect user state
            }
        }
        self.version += 1;
    }

    /// Reconstructs the aggregate state from a series of events.
    pub fn from_events(events: &[IdentityAccessEvent]) -> Self {
        let mut user = User::default();
        for event in events {
            user.apply(event);
        }
        user
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}
