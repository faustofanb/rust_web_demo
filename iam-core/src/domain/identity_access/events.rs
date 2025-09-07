use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents all possible events in the Identity & Access context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IdentityAccessEvent {
    UserRegistered(UserRegistered),
    UserUpdated(UserUpdated),
    UserDeactivated(UserDeactivated),
    RoleCreated(RoleCreated),
    RoleUpdated(RoleUpdated),
    RoleDeleted(RoleDeleted),
    PermissionCreated(PermissionCreated),
    UserRoleAssigned(UserRoleAssigned),
    UserRoleRemoved(UserRoleRemoved),
    RolePermissionGranted(RolePermissionGranted),
    RolePermissionRevoked(RolePermissionRevoked),
}

/// Event indicating that a new user has registered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRegistered {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

/// Event indicating that a user has been updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdated {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Event indicating that a user has been deactivated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDeactivated {
    pub user_id: Uuid,
    pub reason: String,
}

/// Event indicating that a new role has been created.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleCreated {
    pub role_id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

/// Event indicating that a role has been updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleUpdated {
    pub role_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Event indicating that a role has been deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleDeleted {
    pub role_id: Uuid,
}

/// Event indicating that a new permission has been created.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionCreated {
    pub permission_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub permission_type: String, // 'menu', 'button', 'api'
    pub name: String,
    pub code: String,
    pub route: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}

/// Event indicating that a role has been assigned to a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRoleAssigned {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

/// Event indicating that a role has been removed from a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRoleRemoved {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

/// Event indicating that a permission has been granted to a role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RolePermissionGranted {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}

/// Event indicating that a permission has been revoked from a role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RolePermissionRevoked {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}
