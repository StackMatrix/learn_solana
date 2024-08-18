use std::sync::Arc;
use chrono::{Datelike, Utc};
use color_eyre::eyre::{Report, Result};
use rand::{Rng, thread_rng};
use tracing::{error, info};
use crate::core::infrastructure::{jwt::Jwt, InfrastructureLayer};
use crate::core::domain::{
    user::{
        repository_interface::UserRepositoryInterface,
        UserDomain,
        entity::user_entity::Model as UserModel,
        domain_service::UserDomainService
    },
    DomainLayer
};

/// # Description
///     用户应用服务，负责处理用户相关的应用逻辑，如注册、登录、禁用用户和分配角色。
/// # Fields
///     u_domain_serv: Arc<UserDomainService> - 用户仓储接口的引用
///     jwt: Arc<Jwt> - jwt 功能
pub struct UserApplication {
    u_domain_serv: Arc<UserDomainService>,
    jwt: Arc<Jwt>
}

impl UserApplication {
    /// # Description
    ///     创建新的用户应用服务实例
    /// # Params
    ///     infrastructure_layer: Arc<InfrastructureLayer> - 基础设施层的引用，用于获取共享服务如 JWT
    ///     domain_layer: Arc<DomainLayer> - 领域层的引用，用于获取用户领域服务
    /// # Return
    ///     Self: 返回一个新的 `UserApplication` 实例
    pub fn new(infrastructure_layer: Arc<InfrastructureLayer>, domain_layer: Arc<DomainLayer>) -> Self {
        let u_domain_serv = domain_layer.user_domain.domain_service.clone();
        let jwt = infrastructure_layer.jwt.clone();
        Self { u_domain_serv, jwt }
    }


    /// # Description
    ///     注册新用户，通过手机号或邮箱
    /// # Params
    ///     identifier: String - 用户的手机号或邮箱
    ///     password: String - 用户的明文密码
    /// # Return
    ///     Result<(), Report>: 成功返回 Ok() ，失败返回错误信息
    pub async fn register_user(&self, identifier: String, password: String) -> Result<(), Report> {
        // 检查手机号或邮箱是否已经被注册
        if self.u_domain_serv.repository_interface.find_by_mobile_or_email_account(identifier.clone()).await?.is_some() {
            return Err(Report::msg("该手机号或邮箱已被注册"));
        }

        // 判断是手机号还是邮箱注册
        let (email, mobile) = if identifier.contains('@') {
            (identifier, String::new())
        } else {
            (String::new(), identifier)
        };

        // 生成唯一的账号
        let account = self.generate_unique_account().await?;

        // 生成哈希密码
        let hashed_password = match bcrypt::hash(password.clone(), bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(Report::msg("密码哈希生成失败")),
        };

        // 调用领域服务进行用户注册
        let new_user = self.u_domain_serv.create_user(email, mobile, hashed_password, account)?;

        // 保存用户到仓储
        match self.u_domain_serv.repository_interface.save(new_user).await {
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
    ///     生成唯一用户账号，格式为当前年份 + 六位随机数
    /// # Params
    ///     None
    /// # Return
    ///     Result<String, Report>: 成功返回生成的账号，失败返回错误信息
    pub async fn generate_unique_account(&self) -> Result<String, Report> {
        let year = Utc::now().year();
        let mut rng = thread_rng();

        for _ in 0..10 {
            let suffix = rng.gen_range(100000..999999);
            let account = format!("{}{}", year, suffix);

            // 检查账号是否已存在
            if self.u_domain_serv.repository_interface.find_by_account(account.clone()).await?.is_none() {
                return Ok(account);
            }
        }

        Err(Report::msg("无法生成唯一账号"))
    }


    /// # Description
    ///     用户登录，验证用户账号、手机号或邮箱和密码
    /// # Params
    ///     identifier: String - 用户的账号、手机号或邮箱
    ///     password: String - 用户的明文密码
    /// # Return
    ///     Result<String, Report>: 成功返回 JWT 令牌，失败返回错误信息
    pub async fn login_user(&self, identifier: String, password: String) -> Result<String, Report> {
        // 根据账号查找用户
        let user = match self.u_domain_serv.repository_interface.find_by_mobile_or_email_account(identifier).await {
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
    ///     禁用用户，通过 user_id 禁用用户账户
    /// # Params
    ///     user_id: i32 - 要禁用的用户 ID
    /// # Return
    ///     Result<(), Report>: 成功返回 Ok() ，失败返回错误信息
    pub async fn disable_user(&self, user_id: i32) -> Result<(), Report> {
        // 从用户仓库中通过 user_id 查找用户
        let user = match self.u_domain_serv.repository_interface.find_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(Report::msg("用户不存在")),
            Err(e) => return Err(Report::msg(format!("查找用户时出错: {}", e))),
        };

        // 调用领域服务禁用用户
        let updated_user = self.u_domain_serv.disable_user(user)?;

        // 保存用户到仓储
        match self.u_domain_serv.repository_interface.save(updated_user.into()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Report::msg(format!("禁用用户失败: {}", e))),
        }
    }


    /// # Description
    ///     为用户分配角色
    /// # Params
    ///     user_id: i32 - 要分配角色的用户 ID
    ///     role: String - 要分配的角色名称
    /// # Return
    ///     Result<(), Report>: 成功返回 Ok() ，失败返回错误信息
    pub async fn assign_role(&self, user_id: i32, role: String) -> Result<(), Report> {
        // 从用户仓库中通过 user_id 查找用户
        let user = match self.u_domain_serv.repository_interface.find_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(Report::msg("用户不存在")),
            Err(e) => return Err(Report::msg(format!("查找用户时出错: {}", e))),
        };

        // 调用领域服务为用户分配角色
        let updated_user = self.u_domain_serv.assign_role(user, role)?;

        // 保存用户到仓储
        match self.u_domain_serv.repository_interface.save(updated_user.into()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Report::msg(format!("分配角色失败: {}", e))),
        }
    }
}
