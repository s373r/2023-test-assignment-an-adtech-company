use async_trait::async_trait;
use eyre::eyre;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::adapters::api::shared::ApiError;
use crate::adapters::spi::http::HttpBinEntityMapper;
use crate::application::repositories::{DbRequestGroupRepositoryBox, HttpBinRepositoryBox};
use crate::application::use_cases::index::strategies::get_frequent_numbers_asc_strategy;
use crate::application::use_cases::UseCase;
use crate::domain::entities::{RequestEntity, RequestGroupEntity};

type InputNumbers = Vec<u8>;
type FrequentNumbers = Vec<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UseCaseResult {
    pub input_numbers: InputNumbers,
    pub frequent_numbers: FrequentNumbers,
}

impl UseCaseResult {
    pub fn new(input_numbers: InputNumbers, frequent_numbers: FrequentNumbers) -> Self {
        Self {
            input_numbers,
            frequent_numbers,
        }
    }
}

pub struct RunUseCase {
    http_bin_repository: HttpBinRepositoryBox,
    db_request_group_repository: DbRequestGroupRepositoryBox,
}

impl RunUseCase {
    pub fn new(
        http_bin_repository: HttpBinRepositoryBox,
        db_request_group_repository: DbRequestGroupRepositoryBox,
    ) -> Self {
        Self {
            http_bin_repository,
            db_request_group_repository,
        }
    }

    async fn get_responses(&self) -> Vec<RequestEntity> {
        let requests_count = 30u32;
        let requests_data: Vec<_> = (0..requests_count)
            .map(|_| {
                // NOTE: fastrand uses thread-local generator, so we generate all random numbers
                //       in one thread
                let random_value = fastrand::u8(0..=10);

                // NOTE: Dynamic json! macro is used
                //       (instead of strongly-typed HttpBinRequestDto struct)
                //       to illuminate extra conversion to Json::Value (needed for storage into db)
                json!({
                    "value": random_value,
                })
            })
            .collect();

        self.http_bin_repository.send_requests(requests_data).await
    }

    async fn get_numbers(&self, request_entities: &[RequestEntity]) -> InputNumbers {
        request_entities
            .iter()
            .filter_map(|request_entity| {
                // NOTE: We skip all failed responses (as discussed at Telegram)
                //       from the future calculation
                if request_entity.is_successful() {
                    request_entity.response_body.as_ref()
                } else {
                    None
                }
            })
            .filter_map(|response_body| HttpBinEntityMapper::to_model(response_body.clone()))
            .map(|response| response.json.value)
            .collect()
    }
}

#[async_trait(?Send)]
impl UseCase<(), UseCaseResult> for RunUseCase {
    async fn execute(&self, _: ()) -> Result<UseCaseResult, ApiError> {
        let mut request_group_entity = RequestGroupEntity::start_send();

        let responses = self.get_responses().await;

        request_group_entity.finish_send(&responses);

        let numbers = self.get_numbers(&responses).await;

        self.db_request_group_repository
            .insert_with_requests(request_group_entity, responses)
            .await?;

        if numbers.is_empty() {
            const MESSAGE: &str = "No successful responses to process!";

            error!("{MESSAGE}");

            return Err(ApiError::UnexpectedError(eyre!(MESSAGE)));
        }

        let frequent_numbers = get_frequent_numbers_asc_strategy(&numbers);

        Ok(UseCaseResult::new(numbers, frequent_numbers))
    }
}
