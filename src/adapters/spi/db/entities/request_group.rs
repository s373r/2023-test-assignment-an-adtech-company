use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "request_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub started_at: DateTime,
    pub ended_at: Option<DateTime>,
    pub errors_count: Option<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::request::Entity")]
    Request,
}

impl Related<super::request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Request.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
