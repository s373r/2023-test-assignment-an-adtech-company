use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestRepositoryBox;
use crate::application::use_cases::UseCase;
use crate::domain::entities::RequestEntity;

pub struct RequestGetByIdUseCase {
    db_request_repository: DbRequestRepositoryBox,
}

impl RequestGetByIdUseCase {
    pub fn new(db_request_repository: DbRequestRepositoryBox) -> Self {
        Self {
            db_request_repository,
        }
    }
}

type UseCaseArguments = (Uuid,);
type UseCaseResult = Option<RequestEntity>;

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestGetByIdUseCase {
    async fn execute(&self, (id,): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        let request_entity = self.db_request_repository.get_by_id(&id).await?;

        Ok(request_entity)
    }
}
