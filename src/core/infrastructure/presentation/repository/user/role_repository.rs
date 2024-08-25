use sea_orm::{DatabaseConnection};
use std::sync::Arc;

#[allow(dead_code)]
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