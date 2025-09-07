use uuid::Uuid;

/// Command to register a new user.
#[derive(Debug)]
pub struct RegisterUserCommand {
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

/// Command to update an existing user.
#[derive(Debug)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Command to deactivate a user.
#[derive(Debug)]
pub struct DeactivateUserCommand {
    pub user_id: Uuid,
    pub reason: String,
}
