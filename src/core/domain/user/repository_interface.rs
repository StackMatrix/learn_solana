use axum::async_trait;
use color_eyre::Report;
use crate::core::domain::user::entity::user_entity::{ActiveModel, Model as UserModel};

#[async_trait]
pub trait UserRepositoryInterface: Send + Sync {
    async fn save(&self, user: ActiveModel) -> Result<(), Report>;
    async fn find_by_id(&self, id: i32) -> Result<Option<UserModel>, Report>;
    async fn find_by_account(&self, account: String) -> Result<Option<UserModel>, Report>;
    async fn find_by_mobile_or_email_account(&self, identifier: String) -> Result<Option<UserModel>, Report>;
}
