use std::sync::Arc;
use chrono::Utc;
use sea_orm::{ActiveValue, NotSet};
use tracing::{error, info};
use crate::core::domain::user::entity::user_entity::{Model as UserModel, ActiveModel as UserActiveModel};
use crate::core::domain::user::repository::UserRepositoryInterface;

/// # Description
///     用户服务，处理与用户相关的应用逻辑
pub struct UserService {
    repository: Arc<dyn UserRepositoryInterface>,
}

impl UserService {
    /// # Description
    ///     创建新的用户服务
    /// # Param
    ///     repository Arc<dyn UserRepositoryInterface>: 用户仓储接口的引用
    /// # Return
    ///     UserService: 用户服务实例
    pub fn new(repository: Arc<dyn UserRepositoryInterface>) -> Self {
        Self { repository }
    }

    /// # Description
    ///     注册新用户
    /// # Param
    ///     account String: 用户账号
    ///     email String: 用户邮箱
    ///     password String: 用户明文密码
    /// # Return
    ///     Result<User, String>: 成功返回用户实例，失败返回错误信息
    pub async fn register_user(&self, account: String, email: String, password: String) -> Result<(), String> {
        // 创建用户实体
        let hashed_password = match bcrypt::hash(password.clone(), bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err("密码哈希生成失败".into()),
        };

        let new_user = UserActiveModel {
            id: NotSet,
            account: ActiveValue::set(account.to_owned()),
            nickname: Default::default(),
            avatar: Default::default(),
            mobile: ActiveValue::set("1".to_owned()),
            email: ActiveValue::set(email.to_owned()),
            password: ActiveValue::set(hashed_password.to_owned()),
            disable: ActiveValue::set(false),
            level: ActiveValue::set(1),
            reg_type: ActiveValue::set(0),
            created_at: ActiveValue::set(Utc::now()),  // 设置当前时间
            updated_at: ActiveValue::set(Utc::now()),  // 设置当前时间
            deleted_at: Default::default(),
        };

        info!("{}", format!("{:?}", new_user));

        // 保存用户到仓储
        match self.repository.save(new_user).await {
            Ok(_) => {
                info!("用户注册成功");
                Ok(())
            },
            Err(e) => {
                error!("用户注册失败: {:?}", e);
                Err(format!("用户注册失败: {}", e))
            },
        }
    }

    /// # Description
    ///     用户登录
    /// # Param
    ///     account String: 用户账号
    ///     password String: 用户明文密码
    /// # Return
    ///     Result<User, String>: 成功返回用户实例，失败返回错误信息
    pub async fn login_user(&self, account: String, password: String) -> Result<UserModel, String> {
        // 根据账号查找用户
        let user = match self.repository.find_by_account(account).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err("用户不存在".into()),
            Err(e) => return Err(format!("查找用户时出错: {}", e)),
        };

        // 验证密码
        if user.verify_password(password) {
            Ok(user)
        } else {
            Err("密码错误".into())
        }
    }

    /// # Description
    ///     禁用用户
    /// # Param
    ///     user_id i32: 用户ID
    /// # Return
    ///     Result<(), String>: 成功返回空值，失败返回错误信息
    pub async fn disable_user(&mut self, user_id: i32) -> Result<(), String> {
        let mut user = match self.repository.find_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err("用户不存在".into()),
            Err(e) => return Err(format!("查找用户时出错: {}", e)),
        };

        user.disable_user();

        match self.repository.save(user.into()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("禁用用户失败: {}", e)),
        }
    }

    /// # Description
    ///     为用户分配角色
    /// # Param
    ///     user_id i32: 用户ID
    ///     role String: 角色名称
    /// # Return
    ///     Result<(), String>: 成功返回空值，失败返回错误信息
    pub async fn assign_role(&self, user_id: i32, role: String) -> Result<(), String> {
        let mut user = match self.repository.find_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err("用户不存在".into()),
            Err(e) => return Err(format!("查找用户时出错: {}", e)),
        };

        user.level = match role.as_str() {
            "admin" => 10,
            "user" => 1,
            _ => return Err("无效的角色".into()),
        };

        match self.repository.save(user.into()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("分配角色失败: {}", e)),
        }
    }
}
