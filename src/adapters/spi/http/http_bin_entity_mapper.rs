use eyre::{Result, WrapErr};
use uuid::Uuid;

use crate::adapters::spi::http::dtos::HttpBinResponseDto;
use crate::domain::entities::RequestEntity;
use crate::domain::types::DateTime;

pub struct HttpBinEntityMapper {}

impl HttpBinEntityMapper {
    async fn parse_response_body_to_json(response: reqwest::Response) -> Result<serde_json::Value> {
        let response_body = response.text().await?;

        serde_json::from_str(&response_body).wrap_err("JSON parsing failed!")
    }

    pub async fn to_entity(
        request_body: serde_json::Value,
        sent_at: DateTime,
        received_at: DateTime,
        response: Result<reqwest::Response, reqwest::Error>,
    ) -> RequestEntity {
        let mut entity = RequestEntity {
            id: Uuid::new_v4(),
            request_body,
            sent_at,
            received_at: Some(received_at),
            ..Default::default()
        };

        match response {
            Ok(response) => {
                let response_status = response.status();

                entity.response_status = Some(response_status.as_u16());

                if response_status.is_success() {
                    match HttpBinEntityMapper::parse_response_body_to_json(response).await {
                        Ok(json) => entity.response_body = Some(json),
                        Err(e) => entity.error = Some(format!("{e:?}")),
                    }
                }
            }
            Err(e) => {
                entity.response_status = e.status().map(|status| status.as_u16());
                entity.error = Some(format!("{e:?}"))
            }
        }

        entity
    }

    pub fn to_model(response_body: serde_json::Value) -> Option<HttpBinResponseDto> {
        serde_json::from_value(response_body).ok()
    }
}
