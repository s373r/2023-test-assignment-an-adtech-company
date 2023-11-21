use async_trait::async_trait;
use futures::future::join_all;
use std::sync::Arc;

use crate::adapters::spi::http::HttpBinEntityMapper;
use crate::application::repositories::HttpBinRepository;
use crate::domain::entities::RequestEntity;
use crate::shared::utils::datetime::datetime_now;

pub struct HttpBinRepositoryImpl {}

impl HttpBinRepositoryImpl {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl HttpBinRepositoryImpl {
    async fn send_request(
        &self,
        http_client: reqwest::Client,
        request_body: serde_json::Value,
    ) -> RequestEntity {
        let sent_at = datetime_now();
        let response = http_client
            .post("https://httpbin.org/post")
            .json(&request_body)
            .send()
            .await;
        let received_at = datetime_now();

        HttpBinEntityMapper::to_entity(request_body, sent_at, received_at, response).await
    }
}

#[async_trait(?Send)]
impl HttpBinRepository for HttpBinRepositoryImpl {
    async fn send_requests(&self, request_data: Vec<serde_json::Value>) -> Vec<RequestEntity> {
        // NOTE: Use the same client is needed for reusage of TCP connection for requests
        let http_client = reqwest::Client::new();
        let requests_futures = request_data.into_iter().map(|request_data| {
            let http_client = http_client.clone();

            self.send_request(http_client, request_data)
        });

        join_all(requests_futures).await
    }
}
