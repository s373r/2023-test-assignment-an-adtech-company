use crate::application::repositories::{DbRequestGroupRepositoryBox, HttpBinRepositoryBox};
use crate::application::use_cases::index::RunUseCase;

pub struct IndexUseCasesStore {
    pub run: RunUseCase,
}

impl IndexUseCasesStore {
    pub fn new(
        http_bin_repository: HttpBinRepositoryBox,
        db_request_group_repository: DbRequestGroupRepositoryBox,
    ) -> IndexUseCasesStore {
        Self {
            run: RunUseCase::new(http_bin_repository, db_request_group_repository),
        }
    }
}
