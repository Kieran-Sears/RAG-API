use config::Config;

pub fn load_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("./config.json"))
        .add_source(config::Environment::with_prefix("model"))
        .add_source(config::Environment::with_prefix("database"))
        .build()
        .unwrap()
}