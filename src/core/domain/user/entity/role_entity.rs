use sea_orm::{ActiveModelBehavior, DeriveEntityModel, EnumIter, DerivePrimaryKey};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

/// # Description 用户权限表
///     该结构体代表用户权限实体，并映射到数据库中的 role 表。
/// # Param
///     id: 主键，自动递增
///     user_id: 外键，用户ID，关联到用户表
///     permission: 权限名，用于描述用户所拥有的权限
///     created_at: 创建时间
///     updated_at: 更新时间
///     deleted_at: 删除时间（软删除），记录权限删除的时间
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account_id: String,
    pub permission: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Model {
    /// # Description
    ///     检查用户是否拥有特定权限
    /// # Arguments
    ///     permission: 要检查的权限名
    /// # Return
    ///     bool: 如果用户拥有该权限则 true，则否 false
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permission == permission
    }

    /// # Description
    ///     更新权限记录
    ///     更新指定的权限名，保持创建时间和删除时间不变
    /// # Arguments
    ///     new_permission: 新的权限名
    /// # Return
    ///     Result<Self, String>: 返回更新后的权限记录，如果失败则返回错误信息
    pub fn update_permission(&self, new_permission: &str) -> Result<Self, String> {
        Ok(Self {
            id: self.id,
            account_id: self.account_id.clone(),
            permission: new_permission.to_string(),
            created_at: self.created_at,
            updated_at: Utc::now().into(), // 更新 `updated_at` 为当前时间
            deleted_at: self.deleted_at.clone(),
        })
    }

    /// # Description
    ///     创建一个新的权限记录
    ///     在创建时设置 `created_at` 和 `updated_at` 为当前时间
    /// # Arguments
    ///     account_id: 用户ID
    ///     permission: 权限名
    /// # Return
    ///     Self: 新的权限记录实例
    pub fn create_permission(account_id: &str, permission: &str) -> Self {
        Self {
            id: 0, // 默认值，实际插入数据库时由数据库生成
            account_id: account_id.to_string(),
            permission: permission.to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        }
    }

    /// # Description
    ///     软删除权限记录
    ///     设置 `deleted_at` 为当前时间，标记为已删除
    /// # Return
    ///     Self: 更新后的权限记录实例
    pub fn soft_delete(&self) -> Self {
        Self {
            id: self.id,
            account_id: self.account_id.clone(),
            permission: self.permission.clone(),
            created_at: self.created_at,
            updated_at: Utc::now().into(),
            deleted_at: Some(Utc::now().into()),
        }
    }

    /// # Description
    ///     判断权限记录是否已删除
    /// # Return
    ///     bool: 如果 `deleted_at` 不为空则返回 true，否则返回 false
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 用户权限与用户表之间的关系
    // #[sea_orm(has_one = "super::user_entity::Entity")]
    // User,
}

// impl Related<super::user_entity::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}

// impl ActiveModelBehavior for ActiveModel {
//     /// # Description
//     ///     在保存记录之前的操作，自动设置 created_at 和 updated_at 字段
//     /// # Arguments
//     ///     db: 数据库连接
//     ///     insert: 是否是插入操作
//     /// # Return
//     ///     Result<(), DbErr>
//     ///         - Ok: 操作成功
//     ///         - Err: 错误
//     async fn before_save<C>(mut self, _db: &'static C, _insert: bool) -> Result<(), Report>
//     where
//         C: ConnectionTrait
//     {
//         let now = Utc::now();
//         if self.created_at.is_not_set() {
//             self.created_at = ActiveValue::set(now);
//         }
//
//         self.updated_at = ActiveValue::set(now);
//
//         Ok(())
//     }
//
//     /// # Description
//     ///     在删除记录之前的操作，软删除: 设置 deleted_at 字段
//     /// # Arguments
//     ///     db: 数据库连接
//     /// # Return
//     ///     Result<(), DbErr>
//     ///         - Ok: 操作成功
//     ///         - Err: 错误
//     async fn before_delete<C>(&mut self, _db: &'static C) -> Result<(), Report>
//     where
//         C: ConnectionTrait
//     {
//         self.deleted_at = ActiveValue::set(Option::from(Utc::now()));
//
//         Ok(())
//     }
// }