use actix_web::{get, web, HttpResponse};

use crate::adapters::api::shared::{ApiError, AppData};
use crate::application::use_cases::UseCase;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(run);
}

#[get("/run")]
async fn run(app_data: web::Data<AppData>) -> Result<HttpResponse, ApiError> {
    let result = app_data.use_cases.index.run.execute(()).await?;

    Ok(HttpResponse::Ok().json(result))
}
