use async_trait::async_trait;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestRepositoryBox;
use crate::application::use_cases::UseCase;
use crate::domain::entities::RequestEntity;

pub struct RequestGetAllUseCase {
    db_request_repository: DbRequestRepositoryBox,
}

impl RequestGetAllUseCase {
    pub fn new(db_request_repository: DbRequestRepositoryBox) -> Self {
        Self {
            db_request_repository,
        }
    }
}

type UseCaseResult = Vec<RequestEntity>;

#[async_trait(?Send)]
impl UseCase<(), UseCaseResult> for RequestGetAllUseCase {
    async fn execute(&self, _: ()) -> Result<UseCaseResult, ApiError> {
        let request_entities = self.db_request_repository.get_all().await?;

        Ok(request_entities)
    }
}
