use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Deserializer};
use tracing_subscriber::EnvFilter;
use std::{collections::HashMap, env};

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub address: String,
    pub port: String,
    pub request_body_limit: usize,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub address: String,
    pub port: u16,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiConfig,
    pub model: ModelConfig,
    pub database: DatabaseConfig,
    #[serde(deserialize_with = "deserialize_log_levels")]
    pub log_levels: EnvFilter,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("config.json"))
            .add_source(Environment::with_prefix("RAG").try_parsing(true).separator("_"))
            .build()
            .expect("Failed to load configuration")
            .try_deserialize()
    }
}

fn deserialize_log_levels<'de, D>(deserializer: D) -> Result<EnvFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, String> = Deserialize::deserialize(deserializer)?;
    let default = &format!("{}=INFO", env!("CARGO_PKG_NAME").replace("-", "_"));
    let log_levels: String = map.into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join(",");

    EnvFilter::try_new(&format!("{},{}", default, log_levels)).map_err(serde::de::Error::custom)
}
