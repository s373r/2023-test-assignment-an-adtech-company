use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "request")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub request_group_id: Uuid,
    pub sent_at: DateTime,
    pub request_body: Json,
    pub received_at: Option<DateTime>,
    pub response_status: Option<u16>,
    pub response_body: Option<Json>,
    pub error: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::request_group::Entity",
        from = "Column::RequestGroupId",
        to = "super::request_group::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    RequestGroup,
}

impl Related<super::request_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RequestGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
