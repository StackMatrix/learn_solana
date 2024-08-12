use sea_orm::{ActiveModelBehavior, DeriveEntityModel, EnumIter, DerivePrimaryKey};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// # Description 用户表
/// # Param
///     id: 主键，自动递增
///     account: 账号，前四位是年份，后六位随机生成，若相同则重新生成，可用于登录
///     nickname: 昵称
///     avatar: 头像路径
///     mobile: 手机号
///     email: 邮箱
///     password: 密码
///     disable: 账号禁用状态
///     level: 用户级别
///     reg_type: 注册类型
///     created_at: 创建时间
///     updated_at: 更新时间
///     deleted_at: 删除时间（软删除）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub mobile: String,
    pub email: String,
    pub password: String,
    pub disable: bool,
    pub level: i8,
    pub reg_type: i8,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}