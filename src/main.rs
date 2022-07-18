use std::fmt::{Debug, Display};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use rust_service_template::app::AppContext;
use rust_service_template::configuration::{SettingsReader};
use rust_service_template::server::{run_grpc_server, run_http_server};
use rust_service_template::settings_model::SettingsModel;
use rust_service_template::telemetry::{get_subscriber, init_subscriber, ElasticSink};
use tokio::signal;
use tokio::task::JoinError;

#[tokio::main]
async fn main() {
    let settings = SettingsReader::read_settings::<SettingsModel>()
        .await
        .expect("Can't get settings!");
    let endpoint_config = Arc::new(SettingsReader::read_endpoint_settings());
    //ElasticSink::new("127.0.0.1:7878".to_string().parse().unwrap());
    let subscriber = get_subscriber("rust_service_template".into(), "info".into(), move || {
        std::io::stdout() //sink.create_writer()
    });
    init_subscriber(subscriber);

    let app = Arc::new(AppContext::new(&settings));
    //let app_clone = app.clone();
    //JUST A GRPC EXAMPLE
    /* let client_pereodic_task = tokio::spawn(async move {
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
    }); */

    let grpc_server = tokio::spawn(run_grpc_server(endpoint_config.clone(), app.clone()));
    let http_server = tokio::spawn(run_http_server(endpoint_config.clone(), app.clone()));

    tokio::select! {
        o = grpc_server => report_exit("GRPC_SERVER", o),
        o = http_server => report_exit("HTTP_SERVER", o),
        _ = signal::ctrl_c() => {
            println!("Stop signal received!");
            let shut_down = app.states.shutting_down.clone();
            shut_down.store(true, Ordering::Relaxed); },
    };

    //client_pereodic_task.await.unwrap();
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
