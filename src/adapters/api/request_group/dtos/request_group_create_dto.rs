use serde::{Deserialize, Serialize};

use crate::domain::types::DateTime;

#[derive(Serialize, Deserialize)]
pub struct RequestGroupCreateDto {
    pub started_at: DateTime,
    pub ended_at: DateTime,
    pub errors_count: u8,
}
