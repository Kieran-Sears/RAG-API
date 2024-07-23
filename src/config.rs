use config::{*, ext::*};
use std::collections::HashMap;
use tracing_subscriber::EnvFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiConfig {
    address: String,
    port: String,
    request_body_limit: u64,
    log_levels: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ModelConfig {
    path: String,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug, Deserialize)]
struct Configuration {
    api: ApiConfig,
    model: ModelConfig,
    database: DatabaseConfig,
}

pub fn load_config() -> Configuration {
    let config_root = DefaultConfigurationBuilder::new()
        .add_json_file("config.json".is().optional())
        .add_env_vars_with_prefix("RagApi_")
        .add_command_line()
        .build()
        .expect("Failed to load configuration");

    let env_filter = config_root.get_value::<std::collections::HashMap<String, String>>("Api:Log_Levels").unwrap().unwrap();

    let api_config = ApiConfig {
        address: config_root.get("Api:Address").unwrap().to_string(),
        port: config_root.get("Api:Port").unwrap().to_string(),
        request_body_limit: config_root.get_value::<u64>("Api:Request_Body_Limit").unwrap().unwrap(),
        log_levels: env_filter
    };

    let model_config = ModelConfig {

    }

    let db_config = DatabaseConfig {

    }

    Configuration {
        database_url: settings.get("Database:Url").unwrap().to_string(),
        model_path: settings.get("model:path").unwrap().to_string(),
        
        log_filter: env_filter
    } 
}

fn build_env_filter_from_config(log_levels: &HashMap<String, String>) -> EnvFilter {
    let filter = log_levels.into_iter()
        .filter(|(k, _)| *k != "default")
        .map(|(k, v)| format!("{}={}", k, v))
        .fold(format!("{}={}", env!("CARGO_PKG_NAME").replace("-", "_"), "INFO".to_string()), 
              |a, b| format!("{},{}", a, b));

    EnvFilter::try_from_env(filter).unwrap_or_else(|_| EnvFilter::new("INFO"))
}