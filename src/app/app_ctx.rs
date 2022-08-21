use std::sync::Arc;

use tokio::sync::{Mutex};

use crate::{settings_model::SettingsModel, domain::{Database, RequestCounter, DatabaseImpl}};

pub struct AppContext {
    pub states: rust_service_sdk::app::global_states::GlobalStates,
    pub database: Arc<dyn Database<RequestCounter> + Sync + Send>,
    pub some_counter: Arc<Mutex<u64>>,
}

impl AppContext {
    pub fn new(_: &SettingsModel) -> Self {

        Self {
            states: rust_service_sdk::app::global_states::GlobalStates::new(),
            database: Arc::new(DatabaseImpl::new()),
            some_counter: Arc::new(Mutex::new(0))
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


impl rust_service_sdk::app::app_ctx::InitGrpc for AppContext {
    fn init_grpc(
        &self,
        server: Box<std::cell::RefCell<tonic::transport::Server>>,
    ) -> tonic::transport::server::Router {
        
        let bookstore = crate::services::BookStoreImpl::new(self.database.clone());

        server.borrow_mut()
            .add_service(crate::generated_proto::bookstore_server::BookstoreServer::new(bookstore),)
    }
}

