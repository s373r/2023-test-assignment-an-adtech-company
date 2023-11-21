use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::request_group::dtos::RequestGroupUpdateDto;
use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestGroupRepositoryBox;
use crate::application::use_cases::UseCase;

pub struct RequestGroupUpdateUseCase {
    db_request_group_repository: DbRequestGroupRepositoryBox,
}

impl RequestGroupUpdateUseCase {
    pub fn new(db_request_group_repository: DbRequestGroupRepositoryBox) -> Self {
        Self {
            db_request_group_repository,
        }
    }
}

type UseCaseArguments = (Uuid, RequestGroupUpdateDto);
type UseCaseResult = bool;

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestGroupUpdateUseCase {
    async fn execute(&self, (id, dto): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        let has_updated = self.db_request_group_repository.update(&id, &dto).await?;

        Ok(has_updated)
    }
}
