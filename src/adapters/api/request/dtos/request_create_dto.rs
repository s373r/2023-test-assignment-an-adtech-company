use sea_orm::prelude::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::types::DateTime;

#[derive(Serialize, Deserialize)]
pub struct RequestCreateDto {
    pub request_group_id: Uuid,
    pub sent_at: DateTime,
    pub request_body: Json,
    pub received_at: Option<DateTime>,
    pub response_status: Option<u16>,
    pub response_body: Option<Json>,
    pub error: Option<String>,
}
