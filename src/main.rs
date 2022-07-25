use rust_service_template::app::app_ctx::GetGlobalState;
use rust_service_template::app::AppContext;
use rust_service_template::application::Application;
use rust_service_template::configuration::EnvConfig;
use rust_service_template::settings_model::SettingsModel;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::broadcast;
use tokio::task::JoinError;

#[tokio::main]
async fn main() {
    let application = Application
    ::<AppContext, SettingsModel>
    ::init(AppContext::new).await;

    let context = application.context.clone();
    let sink = application.start_logger();
    let (grpc_server, http_server) = application
        .start_hosting(move |server| {
            let bookstore =
                rust_service_template::services::BookStoreImpl::new(context.database.clone());

            server.borrow_mut().add_service(
                rust_service_template::generated_proto::bookstore_server::BookstoreServer::new(
                    bookstore,
                ),
            )
        })
        .await;

    //JUST A GRPC EXAMPLE
    let client_pereodic_task = tokio::spawn(start_test(
        application.context.clone(),
        application.env_config.clone(),
    ));

    let (tx, _) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Stop signal received!");
            application.context.shutting_down();
            tx.send(true).unwrap_or_default();
        },
        o = grpc_server => {report_exit("GRPC_SERVER", o);}
        o = http_server => {report_exit("GRPC_SERVER", o);}
    };

    let shut_thread = tokio::spawn(async move { rx2.recv().await });
    tokio::select! {
        o = client_pereodic_task => {report_exit("EXIT CLIENT", o);}
        _ = shut_thread => {}
    };
    sink.finalize_logs().await;
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
        tokio::time::sleep(std::time::Duration::from_millis(10_000)).await;
        if app.is_shutting_down() {
            println!("STOP CLIENT");
            return Ok(());
        }
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
