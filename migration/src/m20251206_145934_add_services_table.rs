use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_table::Vault;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


manager
            .create_table(
                Table::create()
                    .table(Service::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Service::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Service::VaultId)
                            .uuid()
                            .not_null(),
                    )
                    .col(string(Service::Name).not_null())
                    .col(string(Service::ImageUrl).not_null())
                    .col(
                        ColumnDef::new(Service::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Service::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Service::Table, Service::VaultId)
                            .to(Vault::Table, Vault::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Service::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
enum Service {
    Table,
    Id,
    VaultId,
    Name,
    ImageUrl,
    CreatedAt,
    UpdatedAt,
}
