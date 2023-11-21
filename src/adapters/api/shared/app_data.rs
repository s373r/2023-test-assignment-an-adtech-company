use crate::application::use_cases::UseCasesStore;

pub struct AppData {
    pub api_user: String,
    pub api_password: String,
    pub use_cases: UseCasesStore,
}
