use std::error::Error;
use axum::{routing::get, Router as R};
use crate::core::interfaces::handler::user::UserHandle;

/// # Description
///     【WebServer】路由接口
pub struct Router;

impl Router {
    /// # Description
    ///     路由接口配置
    /// # Param
    ///     None
    /// # Return
    ///     Result<Persistence, Box<dyn Error>>
    ///         - (): None
    ///         - Box<dyn Error>: 错误
    pub async fn new() -> Result<R, Box<dyn Error>> {

        let app = R::new()
            .nest("/v1", Self::v2_routes().await);
            // .merge(Self::auth_routes().await);

        Ok(app)
    }

    /// # Description
    ///     V2 路由组
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    async fn v2_routes() -> R {
        R::new().nest("/userManagement", Self::user_management().await)
    }

    /// # Description
    ///     用户管理路由组
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    async fn user_management() -> R {
        R::new()
            .route("/login", get(UserHandle::login))
            .route("/register", get(UserHandle::register))

    }

}