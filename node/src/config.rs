use std::{fmt::Display, error::Error};

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

#[derive(Debug)]
pub struct ConfigError(config::ConfigError);

impl Error for ConfigError { }

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<config::ConfigError> for ConfigError {
    fn from(value: config::ConfigError) -> Self {
        Self(value)
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = config::Config::builder()
            .add_source(
                File::with_name("config.yaml").format(config::FileFormat::Yaml)
            )
            .add_source(Environment::with_prefix("NODE_"))
            .build()?
            .try_deserialize()?;

        Ok(config)
    }
}