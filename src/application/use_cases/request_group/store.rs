use std::sync::Arc;

use crate::application::repositories::DbRequestGroupRepositoryBox;
use crate::application::use_cases::request_group::*;

pub struct RequestGroupUseCasesStore {
    pub create: RequestGroupCreateUseCase,
    pub delete_by_id: RequestGroupDeleteByIdUseCase,
    pub get_all: RequestGroupGetAllUseCase,
    pub get_by_id: RequestGroupGetByIdUseCase,
    pub update: RequestGroupUpdateUseCase,
}

impl RequestGroupUseCasesStore {
    pub fn new(
        db_request_group_repository: DbRequestGroupRepositoryBox,
    ) -> RequestGroupUseCasesStore {
        Self {
            create: RequestGroupCreateUseCase::new(Arc::clone(&db_request_group_repository)),
            delete_by_id: RequestGroupDeleteByIdUseCase::new(Arc::clone(
                &db_request_group_repository,
            )),
            get_all: RequestGroupGetAllUseCase::new(Arc::clone(&db_request_group_repository)),
            get_by_id: RequestGroupGetByIdUseCase::new(Arc::clone(&db_request_group_repository)),
            update: RequestGroupUpdateUseCase::new(Arc::clone(&db_request_group_repository)),
        }
    }
}
