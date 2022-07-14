use rust_service_template::configuration::SettingsReader;
use rust_service_template::generated_proto::rust_grpc_service;
use rust_service_template::services::BookStoreImpl;
use rust_service_template::settings_model::SettingsModel;
use rust_service_template::telemetry::{get_subscriber, init_subscriber, ElasticSink};
use tonic::transport::Server;
use uuid::Uuid;
use std::time::Duration;
use tracing::{info, Level};
use tracing::instrument;
use rust_grpc_service::bookstore_server::{BookstoreServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut sink = SINK;
    let settings = SettingsReader::read_settings::<SettingsModel>()
        .await
        .expect("Can't get settings!");
    //let sink = std::io::stdout();
    //ElasticSink::new("127.0.0.1:7878".to_string().parse().unwrap());
    let subscriber = 
    get_subscriber(
        "rust_service_template".into(), 
        "info".into(), 
        move || {
        std::io::stdout()//sink.create_writer()
    });
    init_subscriber(subscriber);

    let addr = "127.0.0.1:5001".parse().unwrap();
    let bookstore = BookStoreImpl::default();

    println!("GRPC server listening on {}", addr);
    

    Server::builder()
        .trace_fn(|req| 
            tracing::info_span!(
                "grpc_call", 
                grpc_request = format!("{:?}", req), 
                trace_id = format!("{}",Uuid::new_v4())))
        .add_service(BookstoreServer::new(bookstore))
        .serve(addr)
        .await?;

    Ok(())
}