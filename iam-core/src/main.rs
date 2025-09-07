pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod error;

use crate::application::dtos as user_view;
use crate::application::services::UserService;
use crate::domain::identity_access::RegisterUserCommand;
use crate::infrastructure::persistence::projectors::UserProjector;
use crate::infrastructure::persistence::{EventStore, SqlxEventStore};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use sqlx::MySqlPool;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Setup: Load environment variables and connect to the database
    dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = MySqlPool::connect(&database_url).await?;
    let db_conn: DatabaseConnection = Database::connect(&database_url).await?;

    println!("Database connection successful.");

    // 运行数据库迁移
    sqlx::migrate!("../migrations").run(&pool).await?;

    // --- IMPORTANT SETUP ---
    // Before running, ensure you have created the necessary tables.
    // You can find the SQL DDL in:
    // - `iam-core/src/infrastructure/persistence/event_store.rs` for the `events` table.
    // - `iam-core/src/application/dtos.rs` for the `users_view` table.
    // Make sure to clear the tables before each run for a clean test.
    // -----------------------

    // 2. Instantiate our services and stores
    let event_store = Arc::new(SqlxEventStore::new(pool.clone()));
    let user_service = UserService::new(event_store.clone());
    let user_projector = UserProjector::new(db_conn.clone());

    println!("Services instantiated.");

    // 3. Create a command to register a new user
    let username = format!("testuser_{}", Uuid::new_v4());
    let command = RegisterUserCommand {
        tenant_id: Uuid::new_v4(),
        username: username.clone(),
        email: format!("{}@example.com", username),
        password_hash: "a_very_secure_hash".to_string(),
    };
    println!(
        "\nStep 1: Executing RegisterUserCommand for user '{}'...",
        username
    );

    // 4. (COMMAND SIDE) Execute the command via the service
    let user_id = user_service.register_user(command).await?;
    println!(
        " -> Command successful! User registered with ID: {}",
        user_id
    );
    println!(" -> A 'UserRegistered' event has been saved to the 'events' table.");

    // --- In a real system, a message queue (like Kafka) would bridge this gap ---

    // 5. (QUERY SIDE) Simulate the projector's work
    println!("\nStep 2: Projecting the event to the read model...");

    // 5a. Load the event from the event store (simulating a message consumer)
    let stored_events = event_store.as_ref().load_events(user_id).await?;
    println!(
        " -> Loaded {} event(s) from the event store.",
        stored_events.len()
    );

    // 5b. Handle the event with the projector
    for event in stored_events {
        user_projector.handle_event(&event).await?;
    }
    println!(" -> Projection successful! The 'users_view' table should be updated.");

    // 6. Verify the result by querying the read model
    println!("\nStep 3: Verifying by querying the read model...");
    let user_view: Option<user_view::Model> =
        user_view::Entity::find_by_id(user_id).one(&db_conn).await?;

    println!(" -> Query executed.");

    // 7. Print the final result
    match user_view {
        Some(user) => {
            println!("\n✅ SUCCESS! Found user in the read model:");
            println!("{:#?}", user);
        }
        None => {
            println!(
                "\n❌ FAILURE! User with ID {} not found in the read model.",
                user_id
            );
        }
    }

    Ok(())
}
