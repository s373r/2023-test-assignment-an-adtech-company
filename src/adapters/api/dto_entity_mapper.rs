use uuid::Uuid;

use crate::adapters::api::request::dtos::RequestCreateDto;
use crate::adapters::api::request_group::dtos::RequestGroupCreateDto;
use crate::domain::entities::{RequestEntity, RequestGroupEntity};

pub struct DtoEntityMapper {}

impl DtoEntityMapper {
    pub fn to_request_entity(id: Uuid, dto: RequestCreateDto) -> RequestEntity {
        RequestEntity {
            id,
            request_body: dto.request_body,
            sent_at: dto.sent_at,
            received_at: dto.received_at,
            response_status: dto.response_status,
            response_body: dto.response_body,
            error: dto.error,
        }
    }
    pub fn to_request_group_entity(id: Uuid, dto: RequestGroupCreateDto) -> RequestGroupEntity {
        RequestGroupEntity {
            id,
            started_at: dto.started_at,
            ended_at: Some(dto.ended_at),
            errors_count: Some(dto.errors_count),
        }
    }
}
