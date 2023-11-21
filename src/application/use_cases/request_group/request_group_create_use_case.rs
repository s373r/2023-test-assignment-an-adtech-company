use async_trait::async_trait;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestGroupRepositoryBox;
use crate::application::use_cases::UseCase;
use crate::domain::entities::RequestGroupEntity;

pub struct RequestGroupCreateUseCase {
    db_request_group_repository: DbRequestGroupRepositoryBox,
}

impl RequestGroupCreateUseCase {
    pub fn new(db_request_group_repository: DbRequestGroupRepositoryBox) -> Self {
        Self {
            db_request_group_repository,
        }
    }
}

type UseCaseArguments = (RequestGroupEntity,);
type UseCaseResult = ();

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestGroupCreateUseCase {
    async fn execute(&self, (entity,): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        self.db_request_group_repository.insert(&entity).await?;

        Ok(())
    }
}
