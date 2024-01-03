mod postgres;

use std::{error::Error, fmt::Display};
use crate::{config::Config, authorizer};
use self::postgres::PostgresStartupError;

#[derive(Debug)]
pub enum StartupError {
    Postgres(PostgresStartupError),
}

impl Error for StartupError { }

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartupError::Postgres(e) => e.fmt(f),
        }
    }
}

impl From<PostgresStartupError> for StartupError {
    fn from(value: PostgresStartupError) -> Self {
        Self::Postgres(value)
    }
}

pub async fn on_start(config: &Config) -> Result<(), StartupError> {
    postgres::postgres_start(config).await?;

    authorizer::initialize(&config.authentication);

    Ok(())
}