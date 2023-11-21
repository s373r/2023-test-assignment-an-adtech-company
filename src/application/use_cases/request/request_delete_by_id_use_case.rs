use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestRepositoryBox;
use crate::application::use_cases::UseCase;

pub struct RequestDeleteByIdUseCase {
    db_request_repository: DbRequestRepositoryBox,
}

impl RequestDeleteByIdUseCase {
    pub fn new(db_request_repository: DbRequestRepositoryBox) -> Self {
        Self {
            db_request_repository,
        }
    }
}

type UseCaseArguments = (Uuid,);

type UseCaseResult = bool;

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestDeleteByIdUseCase {
    async fn execute(&self, (id,): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        let has_deleted = self.db_request_repository.delete_by_id(&id).await?;

        Ok(has_deleted)
    }
}
