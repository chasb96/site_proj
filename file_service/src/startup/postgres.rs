use std::{error::Error, fmt::Display};
use crate::{data_stores::postgres::{initialize_connection_pool, InitializeConnectionPoolError, migrate, MigrateError}, config::Config};

#[derive(Debug)]
pub enum PostgresStartupError {
    ConnectionPool(InitializeConnectionPoolError),
    Migration(MigrateError),
}

impl Error for PostgresStartupError { }

impl Display for PostgresStartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresStartupError::ConnectionPool(e) => e.fmt(f),
            PostgresStartupError::Migration(e) => e.fmt(f),
        }
    }
}

impl From<InitializeConnectionPoolError> for PostgresStartupError {
    fn from(value: InitializeConnectionPoolError) -> Self {
        Self::ConnectionPool(value)
    }
}

impl From<MigrateError> for PostgresStartupError {
    fn from(value: MigrateError) -> Self {
        Self::Migration(value)
    }
}

pub async fn postgres_start(config: &Config) -> Result<(), PostgresStartupError> {
    initialize_connection_pool(&config.database)?;

    migrate().await?;

    Ok(())
}