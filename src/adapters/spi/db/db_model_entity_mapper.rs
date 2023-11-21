use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::adapters::spi::db::entities::{request, request_group};
use crate::domain::entities::{RequestEntity, RequestGroupEntity};

pub struct DbModelEntityMapper {}

impl DbModelEntityMapper {
    pub fn to_request_model(
        request_entity: RequestEntity,
        request_group_id: Uuid,
    ) -> request::ActiveModel {
        request::ActiveModel {
            id: Set(request_entity.id),
            request_group_id: Set(request_group_id),
            sent_at: Set(request_entity.sent_at),
            request_body: Set(request_entity.request_body),
            received_at: Set(request_entity.received_at),
            response_status: Set(request_entity.response_status),
            response_body: Set(request_entity.response_body),
            error: Set(request_entity.error),
        }
    }

    pub fn to_request_group_model(
        request_group_entity: &RequestGroupEntity,
    ) -> request_group::ActiveModel {
        request_group::ActiveModel {
            id: Set(request_group_entity.id),
            started_at: Set(request_group_entity.started_at),
            ended_at: Set(request_group_entity.ended_at),
            errors_count: Set(request_group_entity.errors_count),
        }
    }

    pub fn to_request_entity(request_model: request::Model) -> RequestEntity {
        RequestEntity {
            id: request_model.id,
            request_body: request_model.request_body,
            sent_at: request_model.sent_at,
            received_at: request_model.received_at,
            response_status: request_model.response_status,
            response_body: request_model.response_body,
            error: request_model.error,
        }
    }
    pub fn to_request_group_entity(
        request_group_model: &request_group::Model,
    ) -> RequestGroupEntity {
        RequestGroupEntity {
            id: request_group_model.id,
            started_at: request_group_model.started_at,
            ended_at: request_group_model.ended_at,
            errors_count: request_group_model.errors_count,
        }
    }
}
