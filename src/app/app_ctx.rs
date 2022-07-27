use std::sync::Arc;

use crate::{settings_model::SettingsModel, domain::{Database, RequestCounter, DatabaseImpl}};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub states: rust_service_sdk::app::global_states::GlobalStates,
    pub database: Arc<dyn Database<RequestCounter> + Sync + Send>
}

impl AppContext {
    pub fn new(settings: &SettingsModel) -> Self {

        Self {
            states: rust_service_sdk::app::global_states::GlobalStates::new(),
            database: Arc::new(DatabaseImpl::new()),
        }
    }
}

impl rust_service_sdk::app::app_ctx::GetGlobalState for AppContext {
    fn is_initialized(&self) -> bool {
        self.states.is_initialized()
    }

    fn is_shutting_down(&self) -> bool {
        self.states.is_shutting_down()
    }

    fn shutting_down(&self) {
         self.states.shutting_down.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
