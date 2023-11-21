use actix_web::*;
use serde_json::*;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use lib::adapters::api::shared::*;
use lib::application::repositories::*;
use lib::application::use_cases::*;
use lib::domain::entities::*;
use lib::domain::types::*;

pub type RepositoryMocks = (
    Arc<MockHttpBinRepository>,
    Arc<MockDbRequestRepository>,
    Arc<MockDbRequestGroupRepository>,
);

pub fn create_repository_mocks<F>(mock_setup: F) -> RepositoryMocks
where
    F: FnOnce(
        &mut MockHttpBinRepository,
        &mut MockDbRequestRepository,
        &mut MockDbRequestGroupRepository,
    ),
{
    let mut mock_http_bin_repository = MockHttpBinRepository::new();
    let mut mock_db_request_repository = MockDbRequestRepository::new();
    let mut mock_db_request_group_repository = MockDbRequestGroupRepository::new();

    mock_setup(
        &mut mock_http_bin_repository,
        &mut mock_db_request_repository,
        &mut mock_db_request_group_repository,
    );
    (
        Arc::new(mock_http_bin_repository),
        Arc::new(mock_db_request_repository),
        Arc::new(mock_db_request_group_repository),
    )
}

pub fn create_use_cases_store(
    http_bin_repository: Arc<MockHttpBinRepository>,
    db_request_repository: Arc<MockDbRequestRepository>,
    db_request_group_repository: Arc<MockDbRequestGroupRepository>,
) -> UseCasesStore {
    UseCasesStore::new(
        http_bin_repository,
        db_request_repository,
        db_request_group_repository,
    )
}

pub fn create_app_data(repository_mocks: RepositoryMocks) -> web::Data<AppData> {
    web::Data::new(AppData {
        api_user: "test_user".to_string(),
        api_password: "test_password".to_string(),
        use_cases: create_use_cases_store(
            repository_mocks.0,
            repository_mocks.1,
            repository_mocks.2,
        ),
    })
}

pub fn seed_request_entity() -> RequestEntity {
    seed_request_entity_with_value(1)
}

pub fn seed_request_entity_with_value(value: u8) -> RequestEntity {
    RequestEntity {
        id: uuid_predefined(),
        request_body: json!({
            "value": value
        }),
        sent_at: datetime_parse("2023-11-07 15:56:04"),
        received_at: Some(datetime_parse("2023-11-07 15:56:05")),
        response_status: Some(200),
        response_body: Some(json!({
            "json": {
                "value": value
            }
        })),
        error: None,
    }
}

pub fn seed_request_group_entity(id: Uuid) -> RequestGroupEntity {
    RequestGroupEntity {
        id,
        started_at: datetime_parse("2023-11-07 15:56:04"),
        ended_at: Some(datetime_parse("2023-11-07 15:56:05")),
        errors_count: Some(0),
    }
}

pub fn uuid_predefined() -> Uuid {
    Uuid::from_str("00041600-3b4c-46c3-ace0-ed884bebee72").unwrap()
}

pub fn uuid_predefined_for_group_id() -> Uuid {
    Uuid::from_str("001910bc-518d-4308-be40-63f465d96fc1").unwrap()
}

pub fn datetime_parse(value: &str) -> DateTime {
    DateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").unwrap()
}

// NOTE: For "test_user:test_password"
pub const BASIC_AUTH_HEADER: (&str, &str) =
    ("Authorization", "Basic dGVzdF91c2VyOnRlc3RfcGFzc3dvcmQ=");
