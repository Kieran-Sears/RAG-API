use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub folder_path: String,
    pub database: DatabaseConfig,
}


pub fn load_config() -> Result<Config, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./config.json"))
        .add_source(config::Environment::with_prefix("window"))
        .add_source(config::Environment::with_prefix("model"))
        .build();
    
    return settings
}