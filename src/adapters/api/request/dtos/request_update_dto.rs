use sea_orm::prelude::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::types::DateTime;

#[derive(Deserialize, Serialize, Default)]
pub struct RequestUpdateDto {
    pub request_group_id: Option<Uuid>,
    pub sent_at: Option<DateTime>,
    pub request_body: Option<Json>,
    pub received_at: Option<DateTime>,
    pub response_status: Option<u16>,
    pub response_body: Option<Json>,
    pub error: Option<String>,
}
