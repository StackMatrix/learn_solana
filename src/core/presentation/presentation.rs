use std::sync::Arc;
use crate::core::application::ApplicationLayer;
use crate::core::domain::DomainLayer;
use crate::core::infrastructure::InfrastructureLayer;
use crate::core::presentation::handler::Handler;
use crate::core::presentation::router::Router;

pub struct PresentationLayer {
    // pub handler: Arc<Handler>,
    pub domain_layer: Arc<DomainLayer>,
    pub router: Arc<Router>,
}

impl PresentationLayer {
    pub async fn new(infrastructure_layer: Arc<InfrastructureLayer>, domain_layer: Arc<DomainLayer>, application_layer: Arc<ApplicationLayer>) -> Self {
        // let handler = Arc::new(Handler::new(infrastructure_layer.clone(), application_layer.clone()));
        let router = Arc::new(Router::new(infrastructure_layer.clone(), domain_layer.clone(), application_layer.clone()));

        let _get_webserver = infrastructure_layer.webserver.clone().axum_run(router.clone()).await;

        Self { domain_layer, router}
    }
}