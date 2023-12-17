use config::{File, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
}

impl Config {
    pub fn from_env() -> Self {
        config::Config::builder()
            .add_source(
                File::with_name("config.yaml").format(config::FileFormat::Yaml)
            )
            .add_source(Environment::with_prefix("NODE_"))
            .build()
            .expect("Failed to read config")
            .try_deserialize()
            .expect("Failed to deserialize config")
    }
}