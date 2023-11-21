use async_trait::async_trait;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestGroupRepositoryBox;
use crate::application::use_cases::UseCase;
use crate::domain::entities::RequestGroupEntity;

pub struct RequestGroupGetAllUseCase {
    db_request_group_repository: DbRequestGroupRepositoryBox,
}

impl RequestGroupGetAllUseCase {
    pub fn new(db_request_group_repository: DbRequestGroupRepositoryBox) -> Self {
        Self {
            db_request_group_repository,
        }
    }
}

type UseCaseResult = Vec<RequestGroupEntity>;

#[async_trait(?Send)]
impl UseCase<(), UseCaseResult> for RequestGroupGetAllUseCase {
    async fn execute(&self, _: ()) -> Result<UseCaseResult, ApiError> {
        let request_group_entities = self.db_request_group_repository.get_all().await?;

        Ok(request_group_entities)
    }
}
