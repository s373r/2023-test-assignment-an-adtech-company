use async_trait::async_trait;

use crate::adapters::api::shared::ApiError;

#[async_trait(?Send)]
pub trait UseCase<T, R> {
    async fn execute(&self, arguments: T) -> Result<R, ApiError>;
}
