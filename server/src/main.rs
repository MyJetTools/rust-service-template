use rust_service_sdk::app::app_ctx::{GetGlobalState, InitGrpc};
use rust_service_sdk::application::Application;
use rust_service_template::app::AppContext;
use rust_service_template::settings_model::SettingsModel;
use rust_service_template_client::ExampleClientBuilder;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let mut application = Application::<AppContext, SettingsModel>::init(AppContext::new).await;

    let clone = application.context.clone();
    let func = move |server| clone.init_grpc(server);

    let sink = application.start_hosting(func, "rust-service-template".to_string()).await;

    //In case to stop application we can cancel token
    let token = Arc::new(CancellationToken::new());

    // setup custome code

    let task = tokio::spawn(start_test(
        application.context.clone(),
        application.env_config.clone(),
    ));

    let mut running_tasks = vec![task];

    application
        .wait_for_termination(
            sink,
            &mut running_tasks,
            Some(token.clone()),
            graceful_shutdown_func,
            600, // how many msec wail to exeucte graceful_shutdown_func
        )
        .await;
}

async fn graceful_shutdown_func(context: Arc<AppContext>) -> bool {
    let mut guard = context.some_counter.lock().await;
    *guard += 1;
    true
}

async fn start_test(
    app: Arc<AppContext>,
    endpoint: Arc<rust_service_sdk::configuration::EnvConfig>,
) -> Result<(), anyhow::Error> {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(3_333)).await;
        if app.is_shutting_down() {
            println!("STOP CLIENT");
            return Ok(());
        }
        let mut client = ExampleClientBuilder::new(format!(
            "http://{}:{}",
            endpoint.base_url, endpoint.grpc_port
        ))
        .await;

        let request = tonic::Request::new(rust_service_template_generated_proto::GetBookRequest {
            id: "123".into(),
        });

        let response = client.get_book(request).await.unwrap();

        println!("RESPONSE={:?}", response);
    }
}
