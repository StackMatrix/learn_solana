use std::sync::Arc;
use color_eyre::eyre::Result;
use color_eyre::Report;
use tracing::info;
use crate::core::{
    domain::DomainLayer,
    infrastructure::InfrastructureLayer,
};
use crate::core::application::ApplicationLayer;
use crate::core::presentation::PresentationLayer;

/// # Description
///     【app】引导 DDD 每依层次运行
/// # Fields
///     domain_layer  Arc<InfrastructureLayer> - 领域层
///     infrastructure_layer Arc<InfrastructureLayer> - 基础设施层
///     application_layer: Arc<ApplicationLayer> - 基础设施层
///     interface_layer: Arc<InterfaceLayer> - 接口层
#[allow(dead_code)]
pub struct Bootstrap {
    pub infrastructure_layer: Arc<InfrastructureLayer>,
    pub domain_layer: Arc<DomainLayer>,
    pub application_layer: Arc<ApplicationLayer>,
    pub presentation_layer: Arc<PresentationLayer>
}

impl Bootstrap {
    /// # Description
    ///     初始化依赖
    /// # Params
    ///     None
    /// # Return
    ///     Result<Self, Report>
    ///         - Bootstrap: 引导实例化
    ///         - Report: 错误报告
    pub async fn run() -> Result<Self, Report> {
        // 引导基础设施层的启动
        let infrastructure_layer = Arc::new(InfrastructureLayer::new().await?);
        info!("+Bootstrap [InfrastructureLayer] Load complete.");

        // 引导领域层的启动
        let domain_layer = Arc::new(DomainLayer::new().await);
        info!("+Bootstrap [DomainLayer] Load complete.");

        // 引导应用层的启动
        let application_layer = Arc::new(ApplicationLayer::new(infrastructure_layer.clone(), domain_layer.clone()).await);
        info!("+Bootstrap [ApplicationLayer] Load complete.");

        // 引导接口层的启动
        let presentation_layer = Arc::new(PresentationLayer::new(infrastructure_layer.clone(), domain_layer.clone(), application_layer.clone()).await);
        info!("+Bootstrap [InterfaceLayer] Load complete.");

        Ok(Self{ infrastructure_layer, domain_layer, application_layer, presentation_layer })
    }
}