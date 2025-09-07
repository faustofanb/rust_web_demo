#[cfg(test)]
mod user_aggregate_tests {
    use uuid::Uuid;
    use crate::domain::identity_access::aggregates::user::User;
    use crate::domain::identity_access::events::IdentityAccessEvent;

    #[test]
    fn test_user_registration() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();

        let event = User::register(user_id, tenant_id, username.clone(), email.clone(), password_hash.clone())
            .expect("User registration should succeed");

        match event {
            IdentityAccessEvent::UserRegistered(user_registered) => {
                assert_eq!(user_registered.user_id, user_id);
                assert_eq!(user_registered.tenant_id, tenant_id);
                assert_eq!(user_registered.username, username);
                assert_eq!(user_registered.email, email);
                assert_eq!(user_registered.password_hash, password_hash);
            }
            _ => panic!("Expected UserRegistered event"),
        }
    }

    #[test]
    fn test_user_registration_validation() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();

        // Test empty username
        let result = User::register(user_id, tenant_id, "".to_string(), "test@example.com".to_string(), "hash".to_string());
        assert!(result.is_err());

        // Test invalid email
        let result = User::register(user_id, tenant_id, "username".to_string(), "invalid-email".to_string(), "hash".to_string());
        assert!(result.is_err());

        // Test empty password hash
        let result = User::register(user_id, tenant_id, "username".to_string(), "test@example.com".to_string(), "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_user_from_events() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();

        let event = User::register(user_id, tenant_id, username.clone(), email.clone(), password_hash.clone())
            .expect("User registration should succeed");

        let user = User::from_events(&[event]);

        assert_eq!(user.id(), user_id);
        assert_eq!(user.tenant_id(), tenant_id);
        assert_eq!(user.username(), username);
        assert_eq!(user.email(), email);
        assert_eq!(user.version(), 1);
    }

    #[test]
    fn test_user_update() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();

        let register_event = User::register(user_id, tenant_id, username, email, password_hash)
            .expect("User registration should succeed");

        let user = User::from_events(&[register_event]);

        let update_event = user.update(Some("newusername".to_string()), Some("newemail@example.com".to_string()))
            .expect("User update should succeed");

        match update_event {
            IdentityAccessEvent::UserUpdated(user_updated) => {
                assert_eq!(user_updated.user_id, user_id);
                assert_eq!(user_updated.username, Some("newusername".to_string()));
                assert_eq!(user_updated.email, Some("newemail@example.com".to_string()));
            }
            _ => panic!("Expected UserUpdated event"),
        }
    }

    #[test]
    fn test_user_deactivate() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();

        let register_event = User::register(user_id, tenant_id, username, email, password_hash)
            .expect("User registration should succeed");

        let user = User::from_events(&[register_event]);

        let deactivate_event = user.deactivate("User requested deactivation".to_string())
            .expect("User deactivation should succeed");

        match deactivate_event {
            IdentityAccessEvent::UserDeactivated(user_deactivated) => {
                assert_eq!(user_deactivated.user_id, user_id);
                assert_eq!(user_deactivated.reason, "User requested deactivation");
            }
            _ => panic!("Expected UserDeactivated event"),
        }
    }
}
