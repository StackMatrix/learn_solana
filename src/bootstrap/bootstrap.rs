use std::sync::Arc;
use color_eyre::eyre::Result;
use color_eyre::Report;
use tracing::info;
use crate::core::{
    domain::DomainLayer,
    infrastructure::InfrastructureLayer,
};

/// # Description
///     【app】引导 DDD 每依层次运行
/// # Param
///     domain_layer  Arc<InfrastructureLayer>: 领域层
///     infrastructure_layer Arc<InfrastructureLayer>: 基础设施层
pub struct Bootstrap {
    pub infrastructure_layer: Arc<InfrastructureLayer>,
    pub domain_layer: Arc<DomainLayer>,

}

impl Bootstrap {
    /// # Description
    ///     初始化依赖
    /// # Param
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
        let domain_layer = Arc::new(DomainLayer::new(infrastructure_layer.clone()).await);
        info!("+Bootstrap [DomainLayer] Load complete.");




        Ok(Self{ infrastructure_layer, domain_layer })
    }
}