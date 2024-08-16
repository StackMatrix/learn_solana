use sea_orm::{DatabaseConnection};
use sea_orm::entity::prelude::*;
use std::sync::Arc;

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