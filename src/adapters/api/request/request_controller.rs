use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::adapters::api::dto_entity_mapper::DtoEntityMapper;
use crate::adapters::api::request::dtos::{RequestCreateDto, RequestUpdateDto};
use crate::adapters::api::shared::{ApiError, AppData};
use crate::application::use_cases::UseCase;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(get_all_requests)
        .service(get_request_by_id)
        .service(delete_request_by_id)
        .service(create_request)
        .service(update_request);
}

#[get("/")]
async fn get_all_requests(app_data: web::Data<AppData>) -> Result<HttpResponse, ApiError> {
    let request_entities = app_data.use_cases.request.get_all.execute(()).await?;

    if request_entities.is_empty() {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(request_entities))
}

#[get("/{id}")]
async fn get_request_by_id(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let request_entity = app_data.use_cases.request.get_by_id.execute((id,)).await?;

    if let Some(entity) = request_entity {
        return Ok(HttpResponse::Ok().json(entity));
    }

    Ok(HttpResponse::NotFound().finish())
}

#[delete("/{id}")]
async fn delete_request_by_id(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let has_deleted = app_data
        .use_cases
        .request
        .delete_by_id
        .execute((id,))
        .await?;

    if has_deleted {
        return Ok(HttpResponse::Ok().finish());
    }

    Ok(HttpResponse::NotFound().finish())
}

#[put("/{id}")]
async fn create_request(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
    request_body: web::Json<RequestCreateDto>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let dto = request_body.into_inner();
    let request_group_id = dto.request_group_id;
    let entity = DtoEntityMapper::to_request_entity(id, dto);

    app_data
        .use_cases
        .request
        .create
        .execute((request_group_id, entity))
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/{id}")]
async fn update_request(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
    request_body: web::Json<RequestUpdateDto>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let dto = request_body.into_inner();

    app_data.use_cases.request.update.execute((id, dto)).await?;

    Ok(HttpResponse::Ok().finish())
}
