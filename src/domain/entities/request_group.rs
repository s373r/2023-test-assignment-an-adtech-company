use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::RequestEntity;
use crate::domain::types::DateTime;
use crate::shared::utils::datetime::datetime_now;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct RequestGroupEntity {
    pub id: Uuid,
    pub started_at: DateTime,
    pub ended_at: Option<DateTime>,
    pub errors_count: Option<u8>,
}

impl RequestGroupEntity {
    pub fn start_send() -> Self {
        RequestGroupEntity {
            id: Uuid::new_v4(),
            started_at: datetime_now(),
            ..Default::default()
        }
    }

    pub fn finish_send(&mut self, requests: &[RequestEntity]) {
        self.ended_at = Some(datetime_now());

        let errors_count = requests
            .iter()
            .map(|request| request.error.is_some() as u8)
            .sum();

        self.errors_count = Some(errors_count);
    }
}
