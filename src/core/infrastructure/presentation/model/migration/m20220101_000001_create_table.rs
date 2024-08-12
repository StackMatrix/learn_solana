use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::schema::{enumeration_null, pk_auto, string};
use tracing::{info, warn};
use sea_orm::DeriveIden;

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Account,
    Nickname,
    Avatar,
    Mobile,
    Email,
    Password,
    Disable,
    Level,
    RegType,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}


#[derive(DeriveMigrationName)]
pub struct MigratorHandle;

#[async_trait]
impl MigrationTrait for MigratorHandle {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        info!("+ [Migration] create User table starting");

        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(User::Account).string().unique_key().not_null())
                .col(ColumnDef::new(User::Nickname).string())
                .col(ColumnDef::new(User::Avatar).string())
                .col(ColumnDef::new(User::Mobile).string().not_null())
                .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::Disable).boolean().not_null())
                .col(ColumnDef::new(User::Level).tiny_integer().not_null())
                .col(ColumnDef::new(User::RegType).tiny_integer().not_null())
                .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                .col(ColumnDef::new(User::UpdatedAt).date_time().not_null())
                .col(ColumnDef::new(User::DeletedAt).date_time())
                .to_owned(),
        ).await?;

        info!("+ [Migration] create User table complete");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        warn!("+ [Migration] drop User table starting");

        manager.drop_table(
            Table::drop().table(User::Table).if_exists().to_owned()
        ).await?;

        warn!("+ [Migration] drop User table complete");
        Ok(())
    }
}