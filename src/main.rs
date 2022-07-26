use rust_service_template::app::app_ctx::GetGlobalState;
use rust_service_template::app::AppContext;
use rust_service_template::application::Application;
use rust_service_template::configuration::EnvConfig;
use rust_service_template::settings_model::SettingsModel;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let application = Application::<AppContext, SettingsModel>::init(AppContext::new).await;

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
    let mut running_tasks = vec![client_pereodic_task];
    application.wait_for_termination(sink, grpc_server, http_server, &mut running_tasks).await;
}

async fn start_test(
    app: Arc<AppContext>,
    endpoint: Arc<EnvConfig>,
) -> Result<(), anyhow::Error> {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(3_333)).await;
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
