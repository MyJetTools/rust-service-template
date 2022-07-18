use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::{
    app::{AppContext, self}, configuration::EndpointConfig,
    generated_proto::bookstore_server::BookstoreServer, services::BookStoreImpl,
};

pub async fn  run_grpc_server(
    endpoint_config: Arc<EndpointConfig>,
    app_context: Arc<AppContext>,
) -> Result<(), std::io::Error> {
    let addr = format!("{}:{}", endpoint_config.base_url, endpoint_config.grpc_port)
        .parse()
        .unwrap();
    let bookstore = BookStoreImpl::default();

    println!("GRPC server listening on {}", addr);
    tonic::transport::Server::builder()
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

async fn handle(
    _: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    let response = IsAlive {
        version: app::APP_VERSION.to_string()
    };
    let serialized = serde_json::to_string(&response).unwrap();
    Ok(hyper::Response::new(serialized.into()))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IsAlive {
    #[serde(rename = "version")]
    version: String
}

pub async fn run_http_server(
    endpoint_config: Arc<EndpointConfig>,
    app_context: Arc<AppContext>,
) -> Result<(), hyper::Error> {
    let addr = format!("{}:{}", endpoint_config.base_url, endpoint_config.http_port)
        .parse()
        .unwrap();
    println!("HTTP server listening on {}", addr);
    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(handle))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    server.await?;

    Ok(())
}
