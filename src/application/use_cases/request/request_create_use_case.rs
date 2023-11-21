use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestRepositoryBox;
use crate::application::use_cases::UseCase;
use crate::domain::entities::RequestEntity;

pub struct RequestCreateUseCase {
    db_request_repository: DbRequestRepositoryBox,
}

impl RequestCreateUseCase {
    pub fn new(db_request_repository: DbRequestRepositoryBox) -> Self {
        Self {
            db_request_repository,
        }
    }
}

type UseCaseArguments = (Uuid, RequestEntity);
type UseCaseResult = ();

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestCreateUseCase {
    async fn execute(
        &self,
        (request_group_id, entity): UseCaseArguments,
    ) -> Result<UseCaseResult, ApiError> {
        self.db_request_repository
            .insert(&request_group_id, &entity)
            .await?;

        Ok(())
    }
}
