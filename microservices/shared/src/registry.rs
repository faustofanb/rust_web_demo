use consul::{Client, Config};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

use crate::config::ConsulConfig;
use crate::errors::RegistryError;

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    consul: Client,
    service_name: String,
    service_id: String,
    service_address: String,
    service_port: u16,
    health_check_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub health: ServiceHealth,
}

#[derive(Debug, Clone)]
pub enum ServiceHealth {
    Healthy,
    Unhealthy,
    Unknown,
}

impl ServiceRegistry {
    pub fn new(
        consul_config: &ConsulConfig,
        service_name: String,
        service_id: String,
        service_address: String,
        service_port: u16,
    ) -> Result<Self, RegistryError> {
        let config = Config {
            address: consul_config.url.clone(),
            datacenter: Some(consul_config.datacenter.clone()),
            token: consul_config.token.clone(),
            ..Default::default()
        };

        let consul = Client::new(config)?;

        Ok(ServiceRegistry {
            consul,
            service_name,
            service_id,
            service_address,
            service_port,
            health_check_interval: Duration::from_secs(10),
        })
    }

    /// 注册服务到Consul
    pub async fn register(&self) -> Result<(), RegistryError> {
        let registration = consul::catalog::CatalogRegistration {
            node: "rust-node".to_string(),
            address: Some(self.service_address.clone()),
            service: Some(consul::catalog::CatalogService {
                id: Some(self.service_id.clone()),
                service: self.service_name.clone(),
                port: Some(self.service_port),
                tags: Some(vec![
                    "rust".to_string(),
                    "microservice".to_string(),
                    format!("version-{}", env!("CARGO_PKG_VERSION")),
                ]),
                ..Default::default()
            }),
            check: Some(consul::catalog::CatalogCheck {
                http: Some(format!("http://{}:{}/health", self.service_address, self.service_port)),
                interval: Some("10s".to_string()),
                timeout: Some("3s".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        self.consul.catalog().register(&registration).await?;
        info!("服务 {} 注册成功", self.service_name);

        // 启动健康检查
        self.start_health_check().await;

        Ok(())
    }

    /// 注销服务
    pub async fn deregister(&self) -> Result<(), RegistryError> {
        self.consul.catalog().deregister(&self.service_id, None).await?;
        info!("服务 {} 注销成功", self.service_name);
        Ok(())
    }

    /// 发现服务实例
    pub async fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>, RegistryError> {
        let services = self.consul.catalog().service(service_name, None).await?;
        
        let instances = services.into_iter().map(|service| {
            ServiceInstance {
                id: service.service_id.unwrap_or_default(),
                name: service.service_name.unwrap_or_default(),
                address: service.address,
                port: service.service_port.unwrap_or(80),
                tags: service.service_tags.unwrap_or_default(),
                health: ServiceHealth::Unknown, // 需要单独查询健康状态
            }
        }).collect();

        Ok(instances)
    }

    /// 获取健康的服务实例
    pub async fn discover_healthy(&self, service_name: &str) -> Result<Vec<ServiceInstance>, RegistryError> {
        let mut instances = self.discover(service_name).await?;
        
        // 过滤健康的实例
        let healthy_instances: Vec<ServiceInstance> = instances.into_iter()
            .filter(|instance| {
                // 这里可以添加健康检查逻辑
                // 暂时返回所有实例
                true
            })
            .collect();

        Ok(healthy_instances)
    }

    /// 负载均衡选择实例
    pub async fn select_instance(&self, service_name: &str) -> Result<Option<ServiceInstance>, RegistryError> {
        let instances = self.discover_healthy(service_name).await?;
        
        if instances.is_empty() {
            return Ok(None);
        }

        // 简单的轮询负载均衡
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        
        let index = COUNTER.fetch_add(1, Ordering::SeqCst) % instances.len();
        Ok(Some(instances[index].clone()))
    }

    /// 启动健康检查
    async fn start_health_check(&self) {
        let consul = self.consul.clone();
        let service_id = self.service_id.clone();
        let interval = self.health_check_interval;

        tokio::spawn(async move {
            loop {
                sleep(interval).await;
                
                // 这里可以添加自定义的健康检查逻辑
                // 例如检查数据库连接、外部服务等
                match Self::perform_health_check().await {
                    Ok(_) => {
                        // 健康检查通过，可以更新Consul中的健康状态
                        info!("服务 {} 健康检查通过", service_id);
                    }
                    Err(e) => {
                        error!("服务 {} 健康检查失败: {}", service_id, e);
                    }
                }
            }
        });
    }

    /// 执行健康检查
    async fn perform_health_check() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 这里实现具体的健康检查逻辑
        // 例如：检查数据库连接、检查外部依赖等
        Ok(())
    }

    /// 监听服务变化
    pub async fn watch_services<F>(&self, service_name: &str, callback: F) -> Result<(), RegistryError>
    where
        F: Fn(Vec<ServiceInstance>) + Send + Sync + 'static,
    {
        let consul = self.consul.clone();
        let service_name = service_name.to_string();

        tokio::spawn(async move {
            let mut last_services: HashMap<String, ServiceInstance> = HashMap::new();

            loop {
                match consul.catalog().service(&service_name, None).await {
                    Ok(services) => {
                        let current_services: HashMap<String, ServiceInstance> = services
                            .into_iter()
                            .map(|service| {
                                let id = service.service_id.unwrap_or_default();
                                let instance = ServiceInstance {
                                    id: id.clone(),
                                    name: service.service_name.unwrap_or_default(),
                                    address: service.address,
                                    port: service.service_port.unwrap_or(80),
                                    tags: service.service_tags.unwrap_or_default(),
                                    health: ServiceHealth::Unknown,
                                };
                                (id, instance)
                            })
                            .collect();

                        // 检查服务变化
                        if current_services != last_services {
                            let instances: Vec<ServiceInstance> = current_services.values().cloned().collect();
                            callback(instances);
                            last_services = current_services;
                        }
                    }
                    Err(e) => {
                        error!("监听服务 {} 变化失败: {}", service_name, e);
                    }
                }

                sleep(Duration::from_secs(5)).await;
            }
        });

        Ok(())
    }
}
