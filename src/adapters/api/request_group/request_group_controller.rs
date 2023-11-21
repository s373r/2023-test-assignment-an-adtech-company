use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::adapters::api::dto_entity_mapper::DtoEntityMapper;
use crate::adapters::api::request_group::dtos::{RequestGroupCreateDto, RequestGroupUpdateDto};
use crate::adapters::api::shared::{ApiError, AppData};
use crate::application::use_cases::UseCase;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(get_all_request_groups)
        .service(get_request_group_by_id)
        .service(delete_request_group_by_id)
        .service(create_request_group)
        .service(update_request_group);
}

#[get("/")]
async fn get_all_request_groups(app_data: web::Data<AppData>) -> Result<HttpResponse, ApiError> {
    let request_group_entities = app_data.use_cases.request_group.get_all.execute(()).await?;

    if request_group_entities.is_empty() {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(request_group_entities))
}

#[get("/{id}")]
async fn get_request_group_by_id(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let request_group_entity = app_data
        .use_cases
        .request_group
        .get_by_id
        .execute((id,))
        .await?;

    if let Some(entity) = request_group_entity {
        return Ok(HttpResponse::Ok().json(entity));
    }

    Ok(HttpResponse::NotFound().finish())
}

#[delete("/{id}")]
async fn delete_request_group_by_id(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let has_deleted = app_data
        .use_cases
        .request_group
        .delete_by_id
        .execute((id,))
        .await?;

    if has_deleted {
        return Ok(HttpResponse::Ok().finish());
    }

    Ok(HttpResponse::NotFound().finish())
}

#[put("/{id}")]
async fn create_request_group(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
    request_body: web::Json<RequestGroupCreateDto>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let dto = request_body.into_inner();
    let entity = DtoEntityMapper::to_request_group_entity(id, dto);

    app_data
        .use_cases
        .request_group
        .create
        .execute((entity,))
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/{id}")]
async fn update_request_group(
    app_data: web::Data<AppData>,
    path: web::Path<Uuid>,
    request_body: web::Json<RequestGroupUpdateDto>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let dto = request_body.into_inner();

    app_data
        .use_cases
        .request_group
        .update
        .execute((id, dto))
        .await?;

    Ok(HttpResponse::Ok().finish())
}
