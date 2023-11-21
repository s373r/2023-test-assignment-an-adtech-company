use std::sync::Arc;

use crate::application::repositories::DbRequestRepositoryBox;
use crate::application::use_cases::request::*;

pub struct RequestUseCasesStore {
    pub create: RequestCreateUseCase,
    pub delete_by_id: RequestDeleteByIdUseCase,
    pub get_all: RequestGetAllUseCase,
    pub get_by_id: RequestGetByIdUseCase,
    pub update: RequestUpdateUseCase,
}

impl RequestUseCasesStore {
    pub fn new(db_request_repository: DbRequestRepositoryBox) -> RequestUseCasesStore {
        Self {
            create: RequestCreateUseCase::new(Arc::clone(&db_request_repository)),
            delete_by_id: RequestDeleteByIdUseCase::new(Arc::clone(&db_request_repository)),
            get_all: RequestGetAllUseCase::new(Arc::clone(&db_request_repository)),
            get_by_id: RequestGetByIdUseCase::new(Arc::clone(&db_request_repository)),
            update: RequestUpdateUseCase::new(Arc::clone(&db_request_repository)),
        }
    }
}
