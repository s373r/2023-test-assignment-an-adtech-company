use serde::{Deserialize, Serialize};

use crate::domain::types::DateTime;

#[derive(Serialize, Deserialize, Default)]
pub struct RequestGroupUpdateDto {
    pub started_at: Option<DateTime>,
    pub ended_at: Option<DateTime>,
    pub errors_count: Option<u8>,
}
