use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    UnexpectedError(#[from] eyre::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
