use std::{error::Error, fmt::Display};
use crate::{data_store::postgres::{initialize_connection_pool, InitializeConnectionPoolError}, config::Config};

#[derive(Debug)]
pub enum PostgresStartupError {
    ConnectionPool(InitializeConnectionPoolError)
}

impl Error for PostgresStartupError { }

impl Display for PostgresStartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresStartupError::ConnectionPool(e) => e.fmt(f),
        }
    }
}

impl From<InitializeConnectionPoolError> for PostgresStartupError {
    fn from(value: InitializeConnectionPoolError) -> Self {
        Self::ConnectionPool(value)
    }
}

pub fn postgres_start(config: &Config) -> Result<(), PostgresStartupError> {
    initialize_connection_pool(&config.database)
        .map_err(Into::into)
}