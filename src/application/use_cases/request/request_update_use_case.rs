use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::request::dtos::RequestUpdateDto;
use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestRepositoryBox;
use crate::application::use_cases::UseCase;

pub struct RequestUpdateUseCase {
    db_request_repository: DbRequestRepositoryBox,
}

impl RequestUpdateUseCase {
    pub fn new(db_request_repository: DbRequestRepositoryBox) -> Self {
        Self {
            db_request_repository,
        }
    }
}

type UseCaseArguments = (Uuid, RequestUpdateDto);
type UseCaseResult = bool;

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestUpdateUseCase {
    async fn execute(&self, (id, dto): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        let has_updated = self.db_request_repository.update(&id, dto).await?;

        Ok(has_updated)
    }
}
