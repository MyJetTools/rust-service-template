use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::{
    app::{
        app_ctx::{GetGlobalState, GetLogStashUrl},
        AppContext,
    },
    configuration::{EnvConfig, SettingsReader},
    server,
    telemetry::{get_subscriber, init_subscriber, ElasticSink},
};

pub struct Application<TAppContext, TSettingsModel> {
    pub settings: Arc<TSettingsModel>,
    pub context: Arc<TAppContext>,
    pub env_config: Arc<EnvConfig>,
}

impl<TAppContext, TSettingsModel> Application<TAppContext, TSettingsModel>
where
    TAppContext: GetGlobalState + Send + Sync,
    TSettingsModel: DeserializeOwned + GetLogStashUrl,
{
    pub async fn init<TGetConext>(create_context: TGetConext) -> Self
    where
        TGetConext: Fn(&TSettingsModel) -> TAppContext,
    {
        let settings = SettingsReader::read_settings::<TSettingsModel>()
            .await
            .expect("Can't get settings!");

        let env_config = Arc::new(SettingsReader::read_env_settings());
        let context = Arc::new(create_context(&settings));

        Application {
            context,
            env_config,
            settings: Arc::new(settings),
        }
    }

    pub fn start_logger(&self) -> Arc<ElasticSink> {
        let sink = Arc::new(ElasticSink::new(
            self.settings.get_logstash_url().parse().unwrap()
        ));
        let clone = sink.clone();
        let subscriber = get_subscriber(
            "rust_service_template".into(),
            "info".into(),
            move || clone.create_writer(),
            self.env_config.index.clone(),
            self.env_config.environment.clone(),
        );
        init_subscriber(subscriber);
        sink
    }

    pub async fn start_hosting<Func>(
        &self,
        register_services: Func,
    ) -> (tokio::task::JoinHandle<Result<(), std::io::Error>>, tokio::task::JoinHandle<Result<(), hyper::Error>>)
    where
        Func: Fn(Box<std::cell::RefCell<tonic::transport::Server>>) -> tonic::transport::server::Router
            + Send
            + Sync
            + 'static,
    {
        let grpc_server = tokio::spawn(server::run_grpc_server(
            self.env_config.clone(),
            register_services,
        ));
        let http_server = tokio::spawn(server::run_http_server(self.env_config.clone()));

        (grpc_server, http_server)
    }
}
