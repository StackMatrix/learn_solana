use std::sync::Arc;
use color_eyre::{Report, Result};
use crate::core::domain::user::entity::user_entity::{ActiveModel as UserActiveModel, Model as UserModel};
use crate::core::domain::user::repository_interface::UserRepositoryInterface;
use crate::core::infrastructure::InfrastructureLayer;

/// # Description
///     用户领域服务，处理与用户相关的应用逻辑
pub struct UserDomainService {
    pub repository_interface: Arc<dyn UserRepositoryInterface>,
}

impl UserDomainService {
    /// # Description
    ///     创建新的用户服务
    /// # Param
    ///     repository Arc<dyn UserRepositoryInterface>: 用户仓储接口的引用
    ///     jwt_secret String: jwt 密钥
    /// # Return
    ///     UserService: 用户服务实例
    pub fn new(infrastructure_layer: Arc<InfrastructureLayer>) -> Self {
        let repository_interface = infrastructure_layer.persistence.repository.user_repository.clone();
        Self { repository_interface }
    }


    /// # Description
    ///     创建用户,可以是手机号或者是邮箱号
    /// # Param
    ///     email String: 手机号
    ///     mobile String: 邮箱号
    ///     password String: 用户明文密码
    ///     gen_account String: 生成的唯一账号
    /// # Return
    ///     Result<UserActiveModel, Report>: 成功返回用户实例，失败返回错误信息
    pub fn create_user(&self, email: String, mobile: String, password: String, account: String) -> Result<UserActiveModel, Report> {
        // 创建用户实体
        let new_user = UserModel::new(
            account,
            email,
            mobile,
            password,
        );

        Ok(new_user)
    }


    /// # Description
    ///     禁用用户
    ///     disable_user 是一个领域层的业务逻辑，因为禁用用户涉及修改用户状态，应该属于领域层的职责
    /// # Param
    ///     user UserModel: 要禁用的用户
    /// # Return
    ///     Result<UserModel, Report>: 成功返回新的 UserModel，失败返回错误信息
    pub fn disable_user(&self, mut user: UserModel) -> Result<UserModel, Report> {
        user.disable_user();
        Ok(user)
    }

    /// # Description
    ///     为用户分配角色
    /// # Param
    ///     mut user: UserModel, 要设置的用户
    ///     role: String, 要设置的角色名称
    /// # Return
    ///      Result<UserModel, Report>: 成功返回新的 UserModel，失败返回错误信息
    pub fn assign_role(&self, mut user: UserModel, role: String) -> Result<UserModel, Report> {
        user.level = match role.as_str() {
            "admin" => 1,
            "user" => 0,
            _ => return Err(Report::msg("无效的角色")),
        };
        Ok(user)
    }
}
