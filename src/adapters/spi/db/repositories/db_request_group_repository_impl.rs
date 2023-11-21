use async_trait::async_trait;
use eyre::Result;
use futures::future::join_all;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::request_group::dtos::RequestGroupUpdateDto;
use crate::adapters::spi::db::entities::request_group;
use crate::adapters::spi::db::DbModelEntityMapper;
use crate::application::repositories::DbRequestGroupRepository;
use crate::domain::entities::{RequestEntity, RequestGroupEntity};

pub struct DbRequestGroupRepositoryImpl {
    db: DatabaseConnection,
}

impl DbRequestGroupRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Arc<Self> {
        Arc::new(Self { db })
    }
}

#[async_trait(?Send)]
impl DbRequestGroupRepository for DbRequestGroupRepositoryImpl {
    async fn insert_with_requests(
        &self,
        request_group_entity: RequestGroupEntity,
        request_entities: Vec<RequestEntity>,
    ) -> Result<()> {
        let request_group_model =
            DbModelEntityMapper::to_request_group_model(&request_group_entity);
        let request_group_models_iter = request_entities.into_iter().map(|request_entity| {
            DbModelEntityMapper::to_request_model(request_entity, request_group_entity.id)
        });

        {
            let transaction = self.db.begin().await?;

            request_group_model.insert(&transaction).await?;

            let request_model_save_futures =
                request_group_models_iter.map(|request_model| request_model.insert(&transaction));
            let save_results = join_all(request_model_save_futures).await;

            // NOTE: Vec<Result<T, E>> -> Result<Vec<T>, E> conversion -- to be able use ? operator
            save_results.into_iter().collect::<Result<Vec<_>, _>>()?;

            transaction.commit().await?;
        }

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<RequestGroupEntity>> {
        let request_group_models = request_group::Entity::find()
            .order_by_desc(request_group::Column::StartedAt)
            .all(&self.db)
            .await?;
        let request_group_entities: Vec<_> = request_group_models
            .iter()
            .map(DbModelEntityMapper::to_request_group_entity)
            .collect();

        Ok(request_group_entities)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<RequestGroupEntity>> {
        let request_group_model = request_group::Entity::find_by_id(*id).one(&self.db).await?;
        let request_group_entity = request_group_model
            .as_ref()
            .map(DbModelEntityMapper::to_request_group_entity);

        Ok(request_group_entity)
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<bool> {
        let DeleteResult { rows_affected } = request_group::Entity::delete_by_id(*id)
            .exec(&self.db)
            .await?;

        Ok(rows_affected > 0)
    }

    async fn insert(&self, request_group_entity: &RequestGroupEntity) -> Result<()> {
        let request_group_model = DbModelEntityMapper::to_request_group_model(request_group_entity);

        request_group_model.insert(&self.db).await?;

        Ok(())
    }

    async fn update(&self, id: &Uuid, dto: &RequestGroupUpdateDto) -> Result<bool> {
        let request_group_model = request_group::Entity::find_by_id(*id).one(&self.db).await?;

        let Some(model) = request_group_model else {
            return Ok(false);
        };

        let mut active_model = model.into_active_model();

        if let Some(value) = dto.started_at {
            active_model.started_at = Set(value);
        }

        if let Some(value) = dto.ended_at {
            active_model.ended_at = Set(Some(value));
        }

        if let Some(value) = dto.errors_count {
            active_model.errors_count = Set(Some(value));
        }

        active_model.save(&self.db).await?;

        Ok(true)
    }
}
