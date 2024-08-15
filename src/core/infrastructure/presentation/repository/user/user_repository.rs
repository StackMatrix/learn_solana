use crate::core::domain::user::entity::user_entity::{Model as UserModel, Entity as UserEntity, ActiveModel as UserActiveModel, ActiveModel};
use crate::core::domain::user::repository::UserRepositoryInterface;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelTrait;
use async_trait::async_trait;
use std::sync::Arc;
use color_eyre::Report;

pub struct UserRepository {
    db: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub async fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl UserRepositoryInterface for UserRepository {
    /// # Description
    ///     保存用户到数据库。如果用户不存在，则新建；如果用户已存在，则更新
    /// # Param
    ///     user User: 需要保存的用户实体
    /// # Return
    ///     Result<User, Report>: 保存结果，包括保存后的用户模型
    async fn save(&self, user: ActiveModel) -> Result<(), Report> {
        // 显式地指定 ActiveModel 类型
        let mut user_model: UserActiveModel = user.into();

        // 保存用户数据
        user_model.save(self.db.as_ref()).await?;

        Ok(())
    }

    /// # Description
    ///     根据用户ID查找用户
    /// # Param
    ///     id i32: 用户ID
    /// # Return
    ///     Result<Option<User>, Report>: 查找到的用户，如果不存在返回 None
    async fn find_by_id(&self, id: i32) -> Result<Option<UserModel>, Report> {
        let result = UserEntity::find_by_id(id).one(self.db.as_ref()).await;

        Ok(result?)
    }

    /// # Description
    ///     根据账号查找用户
    /// # Param
    ///     account String: 用户账号
    /// # Return
    ///     Result<Option<User>, Report>: 查找到的用户，如果不存在返回 None
    async fn find_by_account(&self, account: String) -> Result<Option<UserModel>, Report> {
        let result = UserEntity::find()
            // 使用完全限定语法消除歧义
            .filter(<UserEntity as EntityTrait>::Column::Account.eq(account))
            .one(self.db.as_ref())
            .await;

        Ok(result?)
    }
}