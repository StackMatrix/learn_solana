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
    RegType,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
    UserID,
    Permission,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Wallet {
    Table,
    Id,
    UserID,
    PubKey,
    PrivyKey,
    Balance,
    Disable,
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
                .col(ColumnDef::new(User::Mobile).string())
                .col(ColumnDef::new(User::Email).string())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::Disable).boolean().not_null())
                .col(ColumnDef::new(User::Level).tiny_integer().not_null())
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
                .col(ColumnDef::new(Role::UserID).integer().not_null())
                .col(ColumnDef::new(Role::Permission).string().not_null())
                .col(ColumnDef::new(Role::CreatedAt).date_time().not_null())
                .col(ColumnDef::new(Role::UpdatedAt).date_time().not_null())
                .col(ColumnDef::new(Role::DeletedAt).date_time())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(Wallet::Table)
                .if_not_exists()
                .col(ColumnDef::new(Wallet::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Wallet::UserID).integer().not_null())
                .col(ColumnDef::new(Wallet::PubKey).string().unique_key().not_null())
                .col(ColumnDef::new(Wallet::PrivyKey).string().unique_key().not_null())
                .col(ColumnDef::new(Wallet::Balance).double().not_null())
                .col(ColumnDef::new(User::Disable).boolean().not_null())
                .col(ColumnDef::new(Wallet::CreatedAt).date_time().not_null())
                .col(ColumnDef::new(Wallet::UpdatedAt).date_time().not_null())
                .col(ColumnDef::new(Wallet::DeletedAt).date_time())
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

        manager.drop_table(
            Table::drop().table(Wallet::Table).if_exists().to_owned()
        ).await?;

        Ok(())
    }
}