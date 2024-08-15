use std::sync::Arc;
use crate::core::domain::user::repository::UserRepositoryInterface;
use crate::core::domain::user::service::UserService;

pub struct UserDomain {
    pub user_service: Arc<UserService>,
}

impl UserDomain {
    /// # Description
    ///     初始化用户领域
    /// # Param
    ///     user_repository Arc<UserRepositoryInterface>: 仓库接口实例
    /// # Return
    ///     Self: 初始化后的用户领域实例
    pub async fn new(user_repository: Arc<dyn UserRepositoryInterface>) -> Self {
        // 初始化用户服务并注入仓库
        let user_service = Arc::new(UserService::new(user_repository));

        Self {
            user_service
        }
    }
}