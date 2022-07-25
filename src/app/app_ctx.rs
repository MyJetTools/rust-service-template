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
}

impl GetGlobalState for AppContext {
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

pub trait GetGlobalState {
    fn is_initialized(&self) -> bool;

    fn is_shutting_down(&self) -> bool;

    fn shutting_down(&self);
}

pub trait GetLogStashUrl {
    fn get_logstash_url(&self) -> String;
}