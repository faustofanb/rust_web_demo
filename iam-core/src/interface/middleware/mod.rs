use std::sync::Arc;

use crate::application::services::UserService;
use crate::infrastructure::persistence::event_store::EventStore;

/// 应用程序状态，包含所有服务依赖
#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub event_store: Arc<dyn EventStore>,
}

impl AppState {
    pub fn new(user_service: Arc<UserService>, event_store: Arc<dyn EventStore>) -> Self {
        Self {
            user_service,
            event_store,
        }
    }
}
