use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // --------------------
        // VAULT TABLE
        // --------------------
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

        // --------------------
        // SERVICES TABLE
        // --------------------
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
                    // Foreign key to vault
                    .col(
                        ColumnDef::new(Service::VaultId)
                            .uuid()
                            .not_null(),
                    )
                    .col(string(Service::Name).not_null())
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
        manager.drop_table(Table::drop().table(Service::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Vault::Table).to_owned()).await
    }
}

// --------------------
// ENUMS FOR COLUMN NAMES
// --------------------

#[derive(Iden)]
enum Vault {
    Table,
    Id,
    ServiceName,
    ServiceUserName,
    ServicePassword,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Service {
    Table,
    Id,
    VaultId,
    Name,
    CreatedAt,
    UpdatedAt,
}
