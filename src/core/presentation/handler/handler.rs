use std::sync::Arc;
use crate::core::application::ApplicationLayer;
use crate::core::infrastructure::InfrastructureLayer;

#[derive(Clone)]
pub struct Handler {
    // pub user_handler: Arc<UserHandle>
}

impl Handler {
    pub fn new(infrastructure_layer: Arc<InfrastructureLayer>, application_layer: Arc<ApplicationLayer>) -> Self {
        // let user_handler = Arc::new(UserHandle::new(infrastructure_layer, application_layer));

        Self {
            // user_handler
        }
    }
}