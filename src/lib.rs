pub mod config;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

// 重新导出常用的类型
pub use config::AppConfig;
pub use errors::{AppError, AppResult};

// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub auth_service: services::AuthService,
    pub user_service: services::UserService,
}
