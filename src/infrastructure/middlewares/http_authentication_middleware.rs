use actix_web::dev::ServiceRequest;
use actix_web::{web, Error};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use eyre::eyre;
use log::{error, warn};

use crate::adapters::api::shared::{ApiError, AppData};

type ValidatorResult = Result<ServiceRequest, (Error, ServiceRequest)>;

pub async fn http_basic_authentication_validator(
    service_request: ServiceRequest,
    credentials: Option<BasicAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let is_public_endpoint = service_request.path() == "/api/v1/run";

    if is_public_endpoint {
        return Ok(service_request);
    }

    let Some(credentials) = credentials else {
        return authentication_error(service_request);
    };

    let app_data = service_request.app_data::<web::Data<AppData>>();
    let Some(app_data) = app_data else {
        let message = "app_data is None!";

        error!("{message}");

        let error = ApiError::UnexpectedError(eyre!(message));

        return Err((error.into(), service_request));
    };

    if has_authenticated(&credentials, &app_data.api_user, &app_data.api_password) {
        return Ok(service_request);
    }

    warn!("Wrong credentials [{credentials:?}]");

    authentication_error(service_request)
}

fn has_authenticated(request_credentials: &BasicAuth, user: &str, password: &str) -> bool {
    if request_credentials.user_id() != user {
        return false;
    }

    let Some(request_password) = request_credentials.password() else {
        return false;
    };

    request_password == password
}

fn authentication_error(service_request: ServiceRequest) -> ValidatorResult {
    let basic_auth_config = Config::default().realm("API testers only");

    Err((
        AuthenticationError::from(basic_auth_config).into(),
        service_request,
    ))
}
