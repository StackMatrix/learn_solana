use sea_orm_migration::prelude::*;
use sea_orm_migration::async_trait::async_trait;
use crate::core::infrastructure::presentation::model::migration::m20220101_000001_create_table;

pub struct MigratorHandle;

#[async_trait]
impl MigratorTrait for MigratorHandle {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::MigratorHandle),
        ]
    }
}
