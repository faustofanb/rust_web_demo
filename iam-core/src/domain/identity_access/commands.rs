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

/// Command to create a new role.
#[derive(Debug)]
pub struct CreateRoleCommand {
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

/// Command to update an existing role.
#[derive(Debug)]
pub struct UpdateRoleCommand {
    pub role_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Command to delete a role.
#[derive(Debug)]
pub struct DeleteRoleCommand {
    pub role_id: Uuid,
}

/// Command to assign a role to a user.
#[derive(Debug)]
pub struct AssignUserRoleCommand {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

/// Command to remove a role from a user.
#[derive(Debug)]
pub struct RemoveUserRoleCommand {
    pub user_id: Uuid,
    pub role_id: Uuid,
}
