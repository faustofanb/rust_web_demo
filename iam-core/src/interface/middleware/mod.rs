use std::sync::Arc;

use crate::application::services::{UserService, QueryService};
use crate::config::AppConfig;
use crate::infrastructure::persistence::event_store::EventStore;

pub mod auth;
pub mod validation;

/// 应用程序状态，包含所有服务依赖
#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub query_service: Arc<QueryService>,
    pub event_store: Arc<dyn EventStore>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(
        user_service: Arc<UserService>,
        query_service: Arc<QueryService>,
        event_store: Arc<dyn EventStore>,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            user_service,
            query_service,
            event_store,
            config,
        }
    }
}
