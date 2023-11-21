use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::api::shared::ApiError;
use crate::application::repositories::DbRequestGroupRepositoryBox;
use crate::application::use_cases::UseCase;
use crate::domain::entities::RequestGroupEntity;

pub struct RequestGroupGetByIdUseCase {
    db_request_group_repository: DbRequestGroupRepositoryBox,
}

impl RequestGroupGetByIdUseCase {
    pub fn new(db_request_group_repository: DbRequestGroupRepositoryBox) -> Self {
        Self {
            db_request_group_repository,
        }
    }
}

type UseCaseArguments = (Uuid,);
type UseCaseResult = Option<RequestGroupEntity>;

#[async_trait(?Send)]
impl UseCase<UseCaseArguments, UseCaseResult> for RequestGroupGetByIdUseCase {
    async fn execute(&self, (id,): UseCaseArguments) -> Result<UseCaseResult, ApiError> {
        let request_group_entity = self.db_request_group_repository.get_by_id(&id).await?;

        Ok(request_group_entity)
    }
}
