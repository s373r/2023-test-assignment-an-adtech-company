use std::sync::Arc;

use crate::application::repositories::{
    DbRequestGroupRepositoryBox, DbRequestRepositoryBox, HttpBinRepositoryBox,
};
use crate::application::use_cases::index::IndexUseCasesStore;
use crate::application::use_cases::request::RequestUseCasesStore;
use crate::application::use_cases::request_group::RequestGroupUseCasesStore;

pub struct UseCasesStore {
    pub index: IndexUseCasesStore,
    pub request: RequestUseCasesStore,
    pub request_group: RequestGroupUseCasesStore,
}

impl UseCasesStore {
    pub fn new(
        http_bin_repository: HttpBinRepositoryBox,
        db_request_repository: DbRequestRepositoryBox,
        db_request_group_repository: DbRequestGroupRepositoryBox,
    ) -> Self {
        Self {
            index: IndexUseCasesStore::new(
                Arc::clone(&http_bin_repository),
                Arc::clone(&db_request_group_repository),
            ),
            request: RequestUseCasesStore::new(Arc::clone(&db_request_repository)),
            request_group: RequestGroupUseCasesStore::new(Arc::clone(&db_request_group_repository)),
        }
    }
}
