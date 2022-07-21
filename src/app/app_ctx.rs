use std::sync::Arc;

use crate::{settings_model::SettingsModel, domain::{Database, RequestCounter, DatabaseImpl}};

use super::global_states::GlobalStates;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub states: GlobalStates,
    pub database: Arc<dyn Database<RequestCounter> + Sync + Send>
}

impl AppContext {
    pub fn new(settings: &SettingsModel) -> Self {

        Self {
            states: GlobalStates::new(),
            database: Arc::new(DatabaseImpl::new()),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.states.is_initialized()
    }

    pub fn is_shutting_down(&self) -> bool {
        self.states.is_shutting_down()
    }
}