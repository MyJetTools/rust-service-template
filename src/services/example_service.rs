use std::sync::Arc;

use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::app::AppContext;
use crate::generated_proto::rust_grpc_service::bookstore_server::Bookstore;
use crate::generated_proto::rust_grpc_service::{GetBookRequest, GetBookResponse};

pub struct BookStoreImpl {
    app: Arc<AppContext>,
}

impl BookStoreImpl {
    pub fn new(app: Arc<AppContext>) -> Self {
        BookStoreImpl { app }
    }
}

#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    #[instrument(skip(self))]
    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        let response = GetBookResponse {
            id: request.into_inner().id,
            author: "Peter".to_owned(),
            name: "Zero to One".to_owned(),
            year: 2014,
            counter: self.app.database.read().await.counter
        };

        self.app.database.increase().await;

        tracing::info!(
            message = "Sending reply.",
            response = format!("{:?}", response)
        );
        Ok(Response::new(response))
    }
}
