use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vault::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vault::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Vault::ServiceName).not_null())
                    .col(string(Vault::ServiceUserName).not_null())
                    .col(string(Vault::ServicePassword).not_null())
                    .col(
                        ColumnDef::new(Vault::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Vault::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vault::Table).to_owned())
            .await
    }
}

// --------------------
// ENUMS FOR COLUMN NAMES
// --------------------

#[derive(Iden)]
pub enum Vault {
    Table,
    Id,
    ServiceName,
    ServiceUserName,
    ServicePassword,
    CreatedAt,
    UpdatedAt,
}
