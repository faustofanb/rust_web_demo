use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Consul错误: {0}")]
    Consul(#[from] consul::Error),
    
    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("服务未找到: {0}")]
    ServiceNotFound(String),
    
    #[error("服务注册失败: {0}")]
    RegistrationFailed(String),
    
    #[error("健康检查失败: {0}")]
    HealthCheckFailed(String),
    
    #[error("内部错误: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type RegistryResult<T> = Result<T, RegistryError>;
