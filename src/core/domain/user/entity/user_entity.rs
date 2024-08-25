use sea_orm::{ActiveModelBehavior, DeriveEntityModel, EnumIter, DerivePrimaryKey, ActiveValue};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use color_eyre::Result;
use chrono::{Utc, DateTime};

/// # Description 用户表
///     该结构体代表用户实体，并映射到数据库中的 `users` 表。
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
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub disable: bool,
    pub level: i8,
    pub reg_type: i8,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Model {
    /// # Description
    ///     创建一个新用户
    /// # Param
    ///     account: String - 系统生成的账号
    ///     email: String - 用户输入的邮箱
    ///     mobile: String - 用户输入的电话
    ///     password: String - 加密后的密码
    /// # Return
    ///     bool: 是否验证通过
    pub fn new(
        account: String,
        email: String,
        mobile: String,
        password: String,
    ) -> ActiveModel {
        // 设置当前时间
        let now_datetime = Utc::now();

        ActiveModel {
            id: ActiveValue::NotSet,
            account: ActiveValue::set(account.to_owned()),
            nickname: Default::default(),
            avatar: Default::default(),
            mobile: ActiveValue::set(Some(mobile)),
            email: ActiveValue::set(Some(email)),
            password: ActiveValue::set(password.to_owned()),
            disable: ActiveValue::set(false),
            level: ActiveValue::set(1),
            reg_type: ActiveValue::set(0),
            created_at: ActiveValue::set(now_datetime),
            updated_at: ActiveValue::set(now_datetime),
            deleted_at: Default::default(),
        }
    }

    /// # Description
    ///     验证用户密码是否正确
    /// # Param
    ///     password: String 用户输入的明文密码
    /// # Return
    ///     bool: 是否验证通过
    pub fn verify_password(&self, password: String) -> bool {
        verify(password, &self.password).unwrap_or_else(|_| false)
    }

    /// # Description
    ///     禁用用户账号
    /// # Return
    ///     ()
    pub fn disable_user(&mut self) {
        self.disable = true;
    }

    /// # Description
    ///     启用用户账号
    /// # Return
    ///     ()
    pub fn enable_user(&mut self) {
        self.disable = false;
    }

    /// # Description
    ///     更新用户的个人信息
    /// # Param
    ///     nickname: Option<String> 更新后的昵称
    ///     avatar: Option<String> 更新后的头像路径
    ///     mobile: Option<String> 更新后的手机号
    /// # Return
    ///     ()
    pub fn update_info(&mut self, nickname: Option<String>, avatar: Option<String>, mobile: Option<String>) {
        if let Some(nick) = nickname {
            self.nickname = Some(nick);
        }
        if let Some(av) = avatar {
            self.avatar = Some(av);
        }
        if let Some(mob) = mobile {
            self.mobile = Some(mob);
        }
    }

    /// # Description
    ///     重置用户密码
    /// # Param
    ///     new_password: String 新密码的明文
    /// # Return
    ///     Result<(), String>: 成功或错误信息
    pub fn reset_password(&mut self, new_password: String) -> Result<(), String> {
        match hash(new_password, DEFAULT_COST) {
            Ok(hashed_password) => {
                self.password = hashed_password;
                Ok(())
            },
            Err(_) => Err("密码哈希生成失败".into()),
        }
    }

    /// # Description
    ///     升级或降级用户级别
    /// # Param
    ///     new_level: i8 用户的新级别
    /// # Return
    ///     ()
    pub fn change_user_level(&mut self, new_level: i8) {
        self.level = new_level;
    }

    /// # Description
    ///     软删除用户
    /// # Return
    ///     ()
    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}