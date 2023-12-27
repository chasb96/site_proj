use std::{error::Error, fmt::Display, sync::OnceLock};
use deadpool::managed::{Pool, BuildError};

use crate::config::DatabaseConfig;

use super::deadpool::ConnectionManager;

static CONNECTION_POOL: OnceLock<Pool<ConnectionManager>> = OnceLock::new();

#[derive(Debug)]
pub enum InitializeConnectionPoolError {
    Sqlx(sqlx::Error),
    Deadpool(BuildError),
}

impl Error for InitializeConnectionPoolError { }

impl Display for InitializeConnectionPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializeConnectionPoolError::Sqlx(e) => e.fmt(f),
            InitializeConnectionPoolError::Deadpool(e) => e.fmt(f),
        }
    }
}

impl From<sqlx::Error> for InitializeConnectionPoolError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<BuildError> for InitializeConnectionPoolError {
    fn from(value: BuildError) -> Self {
        Self::Deadpool(value)
    }
}

pub fn initialize_connection_pool(config: &DatabaseConfig) -> Result<(), InitializeConnectionPoolError> {
    CONNECTION_POOL
        .get_or_try_init(|| {
            let connection_string = config
                .connection_string
                .to_string();

            let manager = ConnectionManager {
                connection_string,
            };

            Pool::builder(manager).build()
        })
        .map(|_| ())
        .map_err(Into::into)
}

pub struct PostgresDatabase {
    pub connection_pool: &'static Pool<ConnectionManager>,
}

impl Default for PostgresDatabase {
    fn default() -> Self {
        Self { 
            connection_pool: CONNECTION_POOL
                .get()
                .expect("Connection Pool used before initialization") 
        }
    }
}