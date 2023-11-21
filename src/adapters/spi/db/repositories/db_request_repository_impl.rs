use async_trait::async_trait;
use eyre::Result;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::request::dtos::RequestUpdateDto;
use crate::adapters::spi::db::entities::request;
use crate::adapters::spi::db::DbModelEntityMapper;
use crate::application::repositories::DbRequestRepository;
use crate::domain::entities::RequestEntity;

pub struct DbRequestRepositoryImpl {
    db: DatabaseConnection,
}

impl DbRequestRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Arc<Self> {
        Arc::new(Self { db })
    }
}

#[async_trait(?Send)]
impl DbRequestRepository for DbRequestRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<RequestEntity>> {
        let request_models = request::Entity::find()
            .order_by_desc(request::Column::SentAt)
            .all(&self.db)
            .await?;
        let request_entities: Vec<_> = request_models
            .into_iter()
            .map(DbModelEntityMapper::to_request_entity)
            .collect();

        Ok(request_entities)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<RequestEntity>> {
        let request_model = request::Entity::find_by_id(*id).one(&self.db).await?;
        let request_entity = request_model.map(DbModelEntityMapper::to_request_entity);

        Ok(request_entity)
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<bool> {
        let DeleteResult { rows_affected } =
            request::Entity::delete_by_id(*id).exec(&self.db).await?;

        Ok(rows_affected > 0)
    }

    async fn insert(&self, request_group_id: &Uuid, request_entity: &RequestEntity) -> Result<()> {
        let request_model =
            DbModelEntityMapper::to_request_model(request_entity.clone(), *request_group_id);

        request_model.insert(&self.db).await?;

        Ok(())
    }

    async fn update(&self, id: &Uuid, dto: RequestUpdateDto) -> Result<bool> {
        let request_model = request::Entity::find_by_id(*id).one(&self.db).await?;

        let Some(model) = request_model else {
            return Ok(false);
        };

        let mut active_model = model.into_active_model();

        if let Some(value) = dto.request_group_id {
            active_model.request_group_id = Set(value);
        }

        if let Some(value) = dto.sent_at {
            active_model.sent_at = Set(value);
        }

        if let Some(value) = dto.request_body {
            active_model.request_body = Set(value);
        }

        if let Some(value) = dto.received_at {
            active_model.received_at = Set(Some(value));
        }

        if let Some(value) = dto.response_status {
            active_model.response_status = Set(Some(value));
        }

        if let Some(value) = dto.response_body {
            active_model.response_body = Set(Some(value));
        }

        if let Some(value) = dto.error {
            active_model.error = Set(Some(value));
        }

        active_model.save(&self.db).await?;

        Ok(true)
    }
}
