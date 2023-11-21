#[cfg(feature = "integration_tests")]
use mockall::{predicate::*, *};

use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::entities::RequestEntity;

pub type HttpBinRepositoryBox = Arc<dyn HttpBinRepository + Sync + Send>;

#[cfg_attr(feature = "integration_tests", automock)]
#[async_trait(?Send)]
pub trait HttpBinRepository {
    async fn send_requests(&self, request_data_iter: Vec<serde_json::Value>) -> Vec<RequestEntity>;
}
