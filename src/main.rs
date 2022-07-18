use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use rust_grpc_service::bookstore_server::BookstoreServer;
use rust_service_template::app::AppContext;
use rust_service_template::configuration::SettingsReader;
use rust_service_template::generated_proto::bookstore_client::BookstoreClient;
use rust_service_template::generated_proto::{rust_grpc_service, GetBookRequest};
use rust_service_template::services::BookStoreImpl;
use rust_service_template::settings_model::SettingsModel;
use rust_service_template::telemetry::{get_subscriber, init_subscriber, ElasticSink};
use tokio::signal;
use tokio::task::JoinError;
use tonic::transport::Server;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let settings = SettingsReader::read_settings::<SettingsModel>()
        .await
        .expect("Can't get settings!");
    //ElasticSink::new("127.0.0.1:7878".to_string().parse().unwrap());
    let subscriber = get_subscriber("rust_service_template".into(), "info".into(), move || {
        std::io::stdout() //sink.create_writer()
    });
    init_subscriber(subscriber);

    let app = Arc::new(AppContext::new(&settings));
    let app_clone = app.clone();
    //JUST A GRPC EXAMPLE
    let client_pereodic_task = tokio::spawn(async move {
        let app = app_clone;
        loop {
            if app.is_shutting_down() {
                println!("STOP CLIENT");
                return;
            }
            tokio::time::sleep(Duration::from_millis(10_000)).await;
            let mut client = BookstoreClient::connect("http://127.0.0.1:5012")
                .await
                .unwrap();

            let request = tonic::Request::new(GetBookRequest { id: "123".into() });

            let response = client.get_book(request).await.unwrap();

            println!("RESPONSE={:?}", response);
        }
    });

    let server = tokio::spawn(run());

    tokio::select! {
        o = server => report_exit("GRPC_SERVER", o),
        o = signal::ctrl_c() => {
            println!("Stop signal received!");
            let shut_down = app.states.shutting_down.clone();
            shut_down.store(true, Ordering::Relaxed); },
    };

    client_pereodic_task.await.unwrap();
}

async fn run() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1:5012".parse().unwrap();
    let bookstore = BookStoreImpl::default();

    println!("GRPC server listening on {}", addr);
    Server::builder()
        .trace_fn(|req| {
            tracing::info_span!(
                "grpc_call",
                grpc_request = format!("{:?}", req),
                trace_id = format!("{}", Uuid::new_v4())
            )
        })
        .add_service(BookstoreServer::new(bookstore))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
