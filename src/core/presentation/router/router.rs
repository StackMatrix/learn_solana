use std::sync::Arc;
use axum::{routing::get, Router as R, middleware, async_trait, debug_handler};
use color_eyre::Report;
use tracing::info;
use crate::core::presentation::handler::user::UserHandle;
use axum::{
    http::{StatusCode},
    response::Response,
    middleware::{Next},
    extract::{Request, Extension},
};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{any, post};
use crate::core::application::ApplicationLayer;
use crate::core::domain::DomainLayer;
use crate::core::infrastructure::InfrastructureLayer;
use crate::core::presentation::handler::Handler;

/// # Description
///     【WebServer】路由接口
pub struct Router {
    pub infrastructure_layer: Arc<InfrastructureLayer>,
    pub domain_layer: Arc<DomainLayer>,
    pub application_layer: Arc<ApplicationLayer>
}

impl Router {
    /// # Description
    ///     路由接口配置
    /// # Param
    ///     None
    /// # Return
    ///     Result<Router, Box<dyn Error>>
    ///         - Router: Router
    ///         - Box<dyn Error>: 错误
    pub fn new(infrastructure_layer: Arc<InfrastructureLayer>, domain_layer: Arc<DomainLayer>, application_layer: Arc<ApplicationLayer>) -> Self {
        Self { infrastructure_layer: infrastructure_layer.clone(), domain_layer: domain_layer.clone(), application_layer: application_layer.clone() }
    }

    /// # Description
    ///     V1 路由组
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn v1_routes(&self) -> R {
        R::new()
            .nest("/v1", R::new().nest("/userManagement", self.user_management().await))

    }

    /// # Description
    ///     用户管理路由组
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    async fn user_management(&self) -> R {
        R::new()
            .route("/register", get(UserHandle::register))
            .route("/login", get(UserHandle::login))
            .with_state(self.application_layer.clone())

    }
}


/// # Description
///     认证中间件
pub struct AuthMiddleware;

impl AuthMiddleware {
    /// # Description
    ///     认证
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn auth() {
        info!("+ [Router] Register handle");
    }

    pub async fn my_middleware(
        request: Request,
        next: Next,
    ) -> Response {
        // do something with `request`...
        info!("+ [Router] Register handle");

        let response = next.run(request).await;

        // do something with `response`...

        response
    }
}

// .route_layer(middleware::from_fn(AuthMiddleware::my_middleware));