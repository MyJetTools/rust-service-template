use crate::settings_model::SettingsModel;

use super::global_states::GlobalStates;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub states: GlobalStates,
}

impl AppContext {
    pub fn new(settings: &SettingsModel) -> Self {

        Self {
            states: GlobalStates::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.states.is_initialized()
    }

    pub fn is_shutting_down(&self) -> bool {
        self.states.is_shutting_down()
    }
}