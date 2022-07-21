use rust_service_template::app::AppContext;
use rust_service_template::configuration::{EnvConfig, SettingsReader};
use rust_service_template::server::{run_grpc_server, run_http_server};
use rust_service_template::settings_model::SettingsModel;
use rust_service_template::telemetry::{get_subscriber, init_subscriber, ElasticSink};
use std::fmt::{format, Debug, Display};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::signal;
use tokio::task::JoinError;

#[tokio::main]
async fn main() {
    let settings = SettingsReader::read_settings::<SettingsModel>()
        .await
        .expect("Can't get settings!");

    let app = Arc::new(AppContext::new(&settings));

    let env_config = Arc::new(SettingsReader::read_env_settings());
    let sink = ElasticSink::new(
        "192.168.70.8:5044".to_string().parse().unwrap(),
        app.clone(),
    );
    let subscriber = get_subscriber(
        "rust_service_template".into(),
        "info".into(),
        move || sink.create_writer(),
        env_config.index.clone(),
        env_config.environment.clone(),
    );
    init_subscriber(subscriber);
    //JUST A GRPC EXAMPLE
    let client_pereodic_task = tokio::spawn(start_test(app.clone(), env_config.clone()));

    let grpc_server = tokio::spawn(run_grpc_server(env_config.clone(), app.clone()));
    let http_server = tokio::spawn(run_http_server(env_config.clone(), app.clone()));

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Stop signal received!");
            let shut_down = app.states.shutting_down.clone();
            shut_down.store(true, Ordering::Relaxed);
        },
        o = grpc_server => {report_exit("GRPC_SERVER", o);}
        o = http_server => {report_exit("GRPC_SERVER", o);}
    };

    client_pereodic_task.await.unwrap();
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

async fn start_test(
    app: Arc<AppContext>,
    endpoint: Arc<EnvConfig>,
) -> Result<(), tonic::transport::Error> {
    loop {
        if app.is_shutting_down() {
            println!("STOP CLIENT");
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(10_000)).await;
        let mut client =
            rust_service_template::generated_proto::bookstore_client::BookstoreClient::connect(
                format!("http://{}:{}", endpoint.base_url, endpoint.grpc_port),
            )
            .await?;

        let request = tonic::Request::new(rust_service_template::generated_proto::GetBookRequest {
            id: "123".into(),
        });

        let response = client.get_book(request).await.unwrap();

        println!("RESPONSE={:?}", response);
    }
}
