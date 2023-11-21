use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestGroupRepositoryBox;
use crate::application::use_cases::UseCase;

pub struct RequestGroupDeleteByIdUseCase {
    db_request_group_repository: DbRequestGroupRepositoryBox,
}

impl RequestGroupDeleteByIdUseCase {
    pub fn new(db_request_group_repository: DbRequestGroupRepositoryBox) -> Self {
        Self {
            db_request_group_repository,
        }
    }
}

type UseCaseArguments = (Uuid,);
type UseCaseResult = bool;

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestGroupDeleteByIdUseCase {
    async fn execute(&self, (id,): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        let has_deleted = self.db_request_group_repository.delete_by_id(&id).await?;

        Ok(has_deleted)
    }
}
