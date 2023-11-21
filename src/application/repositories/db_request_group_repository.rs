#[cfg(feature = "integration_tests")]
use mockall::{predicate::*, *};

use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::request_group::dtos::RequestGroupUpdateDto;
use crate::domain::entities::{RequestEntity, RequestGroupEntity};

pub type DbRequestGroupRepositoryBox = Arc<dyn DbRequestGroupRepository + Sync + Send>;

#[cfg_attr(feature = "integration_tests", automock)]
#[async_trait(?Send)]
pub trait DbRequestGroupRepository {
    async fn insert_with_requests(
        &self,
        request_group_entity: RequestGroupEntity,
        request_entities: Vec<RequestEntity>,
    ) -> Result<()>;

    async fn get_all(&self) -> Result<Vec<RequestGroupEntity>>;

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<RequestGroupEntity>>;

    async fn delete_by_id(&self, id: &Uuid) -> Result<bool>;

    async fn insert(&self, request_group_entity: &RequestGroupEntity) -> Result<()>;

    async fn update(&self, id: &Uuid, dto: &RequestGroupUpdateDto) -> Result<bool>;
}
