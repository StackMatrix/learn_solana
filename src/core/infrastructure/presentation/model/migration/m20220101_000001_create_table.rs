use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;
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
    Role,
    RegType,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
    AccountID,
    Permission,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveMigrationName)]
pub struct MigratorHandle;

#[async_trait]
impl MigrationTrait for MigratorHandle {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                .col(ColumnDef::new(User::Role).string().not_null())
                .col(ColumnDef::new(User::RegType).tiny_integer().not_null())
                .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                .col(ColumnDef::new(User::UpdatedAt).date_time().not_null())
                .col(ColumnDef::new(User::DeletedAt).date_time())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(Role::Table)
                .if_not_exists()
                .col(ColumnDef::new(Role::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Role::AccountID).string().unique_key().not_null())
                .col(ColumnDef::new(Role::CreatedAt).date_time().not_null())
                .col(ColumnDef::new(Role::UpdatedAt).date_time().not_null())
                .col(ColumnDef::new(Role::DeletedAt).date_time())
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(User::Table).if_exists().to_owned()
        ).await?;


        manager.drop_table(
            Table::drop().table(Role::Table).if_exists().to_owned()
        ).await?;
        Ok(())
    }
}