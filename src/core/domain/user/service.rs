use std::future::Future;
use std::sync::Arc;
use chrono::{Datelike, Utc};
use sea_orm::{ActiveValue, NotSet};
use tracing::{error, info};
use color_eyre::{Report, Result};
use rand::{Rng, thread_rng};
use crate::core::domain::user::entity::user_entity::{Model as UserModel, ActiveModel as UserActiveModel, Model};
use crate::core::domain::user::repository::UserRepositoryInterface;
use crate::core::infrastructure::config::JwtConfig;
use crate::core::infrastructure::InfrastructureLayer;
use crate::core::infrastructure::jwt::Jwt;

/// # Description
///     用户服务，处理与用户相关的应用逻辑
pub struct UserService {
    repository: Arc<dyn UserRepositoryInterface>,
    jwt: Arc<Jwt>
}

impl UserService {
    /// # Description
    ///     创建新的用户服务
    /// # Param
    ///     repository Arc<dyn UserRepositoryInterface>: 用户仓储接口的引用
    ///     jwt_secret String: jwt 密钥
    /// # Return
    ///     UserService: 用户服务实例
    pub fn new(infrastructure_layer: Arc<InfrastructureLayer>) -> Self {
        let repository = infrastructure_layer.persistence.repository.user_repository.clone();
        let jwt = infrastructure_layer.jwt.clone();
        Self { repository, jwt }
    }

    /// # Description
    ///     生成唯一用户账号
    /// # Return
    ///     String: 生成的账号
    pub async fn generate_unique_account(&self) -> Result<String, Report> {
        let year = Utc::now().year();
        let mut rng = thread_rng();

        for _ in 0..10 {
            let suffix = rng.gen_range(100000..999999);
            let account = format!("{}{}", year, suffix);

            // 检查账号是否已存在
            if self.repository.find_by_account(account.clone()).await?.is_none() {
                return Ok(account);
            }
        }

        Err(Report::msg("无法生成唯一账号"))
    }

    /// # Description
    ///     注册新用户
    /// # Param
    ///     identifier String: 被验证的字段
    ///     password String: 用户明文密码
    /// # Return
    ///     Result<User, Report>: 成功返回用户实例，失败返回错误信息
    pub async fn register_user(&self, identifier: String, password: String) -> Result<(), Report> {
        // 检查手机号或邮箱是否已经被注册
        if self.repository.find_by_mobile_or_email_account(identifier.clone()).await?.is_some() {
            return Err(Report::msg("该手机号或邮箱已被注册"));
        }

        // 生成账号
        let account = self.generate_unique_account().await?;

        // 创建用户实体
        let hashed_password = match bcrypt::hash(password.clone(), bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(Report::msg("密码哈希生成失败")),
        };

        // 判断是手机号还是邮箱注册
        let (email, mobile)= if identifier.contains('@') {
            (ActiveValue::set(Some(identifier.clone())), Default::default())
        } else {
            (Default::default(), ActiveValue::set(Some(identifier.clone())))
        };

        // 实例化用户模型
        let new_user = UserActiveModel {
            id: NotSet,
            account: ActiveValue::set(account.to_owned()),
            nickname: Default::default(),
            avatar: Default::default(),
            mobile,
            email,
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
                Err(Report::msg("用户注册失败"))
            },
        }
    }

    /// # Description
    ///     用户登录
    /// # Param=
    ///     identifier String: 被验证的用户账号｜手机号｜邮箱号字段
    ///     password String: 用户明文密码
    /// # Return
    ///     Result<User, String>: 成功返回用户实例，失败返回错误信息
    pub async fn login_user(&self, identifier: String, password: String) -> Result<String, Report> {
        // 根据账号查找用户
        let user = match self.repository.find_by_mobile_or_email_account(identifier).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(Report::msg("用户不存在")),
            Err(e) => return Err(Report::msg(format!("查找用户时出错: {}", e))),
        };

        // 验证密码
        if user.verify_password(password) {
            // 生成 JWT
            let token = self.jwt.create_jwt(&user.id.to_string());

            Ok(token)
        } else {
            Err(Report::msg("密码错误"))
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
