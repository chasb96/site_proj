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
            InitializeConnectionPoolError::Sqlx(e) => write!(f, "InitializeConnectionPoolError::Sqlx({})", e),
            InitializeConnectionPoolError::Deadpool(e) => write!(f, "InitializeConnectionPoolError::Deadpool({})", e),
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

#[derive(Debug)]
pub enum MigrateError {
    Sqlx(sqlx::migrate::MigrateError),
    Deadpool(deadpool::managed::PoolError<sqlx::error::Error>)
}

impl Error for MigrateError { }

impl Display for MigrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrateError::Sqlx(e) => write!(f, "MigrateError::Sqlx({})", e),
            MigrateError::Deadpool(e) => write!(f, "MigrateError::Deadpool({})", e),
        }
    }
}

impl From<sqlx::migrate::MigrateError> for MigrateError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<deadpool::managed::PoolError<sqlx::error::Error>> for MigrateError {
    fn from(value: deadpool::managed::PoolError<sqlx::error::Error>) -> Self {
        Self::Deadpool(value)
    }
}

pub async fn migrate() -> Result<(), MigrateError> {
    let mut conn = CONNECTION_POOL
        .get()
        .expect("Connection Pool used before initialization") 
        .get()
        .await?;

    sqlx::migrate!("./migrations")
        .run(conn.as_mut())
        .await?;

    Ok(())
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