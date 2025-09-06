use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroserviceConfig {
    pub service: ServiceConfig,
    pub consul: ConsulConfig,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub version: String,
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsulConfig {
    pub url: String,
    pub datacenter: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl MicroserviceConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok();

        let service_name = env::var("SERVICE_NAME")
            .unwrap_or_else(|_| "rust-microservice".to_string());

        let service_version = env::var("SERVICE_VERSION")
            .unwrap_or_else(|_| "1.0.0".to_string());

        let service_port = env::var("SERVICE_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| config::ConfigError::Message("Invalid SERVICE_PORT".to_string()))?;

        let service_host = env::var("SERVICE_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let consul_url = env::var("CONSUL_URL")
            .unwrap_or_else(|_| "http://localhost:8500".to_string());

        let consul_datacenter = env::var("CONSUL_DATACENTER")
            .unwrap_or_else(|_| "dc1".to_string());

        let consul_token = env::var("CONSUL_TOKEN").ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/microservice".to_string());

        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .map_err(|_| config::ConfigError::Message("Invalid DATABASE_MAX_CONNECTIONS".to_string()))?;

        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| config::ConfigError::Message("Invalid SERVER_PORT".to_string()))?;

        Ok(MicroserviceConfig {
            service: ServiceConfig {
                name: service_name,
                version: service_version,
                port: service_port,
                host: service_host,
            },
            consul: ConsulConfig {
                url: consul_url,
                datacenter: consul_datacenter,
                token: consul_token,
            },
            database: DatabaseConfig {
                url: database_url,
                max_connections,
            },
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
        })
    }
}
