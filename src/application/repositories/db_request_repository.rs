#[cfg(feature = "integration_tests")]
use mockall::{predicate::*, *};

use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::request::dtos::RequestUpdateDto;
use crate::domain::entities::RequestEntity;

pub type DbRequestRepositoryBox = Arc<dyn DbRequestRepository + Sync + Send>;

#[cfg_attr(feature = "integration_tests", automock)]
#[async_trait(?Send)]
pub trait DbRequestRepository {
    async fn get_all(&self) -> Result<Vec<RequestEntity>>;

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<RequestEntity>>;

    async fn delete_by_id(&self, id: &Uuid) -> Result<bool>;

    async fn insert(&self, request_group_id: &Uuid, request_entity: &RequestEntity) -> Result<()>;

    async fn update(&self, id: &Uuid, dto: RequestUpdateDto) -> Result<bool>;
}
