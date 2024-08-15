use crate::core::domain::user::entity::user_entity::{Model as UserModel, Entity as UserEntity, ActiveModel as UserActiveModel, ActiveModel};
use crate::core::domain::user::repository::UserRepositoryInterface;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelTrait;
use async_trait::async_trait;
use std::sync::Arc;
use color_eyre::Report;

pub struct RoleRepository {
    db: Arc<DatabaseConnection>,
}

impl RoleRepository {
    pub async fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

// #[async_trait]
// impl RoleRepositoryInterface for RoleRepository {
//
// }