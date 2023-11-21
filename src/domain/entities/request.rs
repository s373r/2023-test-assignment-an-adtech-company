use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::types::DateTime;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestEntity {
    pub id: Uuid,
    pub request_body: serde_json::Value,
    pub sent_at: DateTime,
    pub received_at: Option<DateTime>,
    pub response_status: Option<u16>,
    pub response_body: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl Clone for RequestEntity {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            request_body: self.request_body.clone(),
            sent_at: self.sent_at,
            received_at: self.received_at,
            response_status: self.response_status,
            response_body: self.response_body.clone(),
            error: self.error.clone(),
        }
    }
}

impl RequestEntity {
    pub fn is_successful(&self) -> bool {
        self.response_status
            .map_or(false, |status| status == StatusCode::OK.as_u16())
    }
}
