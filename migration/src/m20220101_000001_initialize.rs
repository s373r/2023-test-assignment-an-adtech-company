use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // NOTE: ColumnDef::new(...).date_time() (or DATETIME in DSL terms)
        //       has no milliseconds in MySQL
        let datetime_with_milliseconds = ColumnType::custom("DATETIME(3)");

        manager
            .create_table(
                Table::create()
                    .table(RequestGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RequestGroup::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new_with_type(
                            RequestGroup::StartedAt,
                            datetime_with_milliseconds.clone(),
                        )
                        .not_null(),
                    )
                    .col(&mut ColumnDef::new_with_type(
                        RequestGroup::EndedAt,
                        datetime_with_milliseconds.clone(),
                    ))
                    .col(ColumnDef::new(RequestGroup::ErrorsCount).tiny_unsigned())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Request::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Request::Id).uuid().not_null().primary_key(), // .extra("DEFAULT (UUID_TO_BIN(UUID()))"),
                    )
                    .col(ColumnDef::new(Request::RequestGroupId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Request::Table, Request::RequestGroupId)
                            .to(RequestGroup::Table, RequestGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new_with_type(
                            Request::SentAt,
                            datetime_with_milliseconds.clone(),
                        )
                        .not_null(),
                    )
                    .col(ColumnDef::new(Request::RequestBody).json().not_null())
                    .col(&mut ColumnDef::new_with_type(
                        Request::ReceivedAt,
                        datetime_with_milliseconds.clone(),
                    ))
                    .col(ColumnDef::new(Request::ResponseStatus).small_unsigned())
                    .col(ColumnDef::new(Request::ResponseBody).json())
                    .col(ColumnDef::new(Request::Error).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RequestGroup::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Request::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RequestGroup {
    Table,
    Id,
    StartedAt,
    EndedAt,
    ErrorsCount,
}

#[derive(DeriveIden)]
enum Request {
    Table,
    Id,
    RequestGroupId,
    SentAt,
    RequestBody,
    ReceivedAt,
    ResponseStatus,
    ResponseBody,
    Error,
}
