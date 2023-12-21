use std::{error::Error, fmt::Display, sync::OnceLock};
use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;
use crate::config::DatabaseConfig;

static CONNECTION_POOL: OnceLock<Pool<ConnectionManager<PgConnection>>> = OnceLock::new();

#[derive(Debug)]
pub struct InitializeConnectionPoolError(r2d2::Error);

impl Error for InitializeConnectionPoolError { }

impl Display for InitializeConnectionPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<r2d2::Error> for InitializeConnectionPoolError {
    fn from(value: r2d2::Error) -> Self {
        Self(value)
    }
}

pub fn initialize_connection_pool(config: &DatabaseConfig) -> Result<(), InitializeConnectionPoolError> {
    CONNECTION_POOL
        .get_or_try_init(|| {
            let connection_string = config
                .connection_string
                .to_string();

            let manager: ConnectionManager<PgConnection> = 
                ConnectionManager::new(connection_string);

            Pool::builder()
                .build(manager)
        })
        .map(|_| ())
        .map_err(Into::into)
}

pub struct PostgresDataStore {
    pub connection_pool: &'static Pool<ConnectionManager<PgConnection>>,
}

impl Default for PostgresDataStore {
    fn default() -> Self {
        Self { 
            connection_pool: CONNECTION_POOL
                .get()
                .expect("Application skipped startup process") 
        }
    }
}

diesel::table! {
    users (id) {
        id -> Serial,
        username -> VarChar,
        password_hash -> VarChar,
    }
}

diesel::table! {
    nodes (id) {
        id -> Serial,
        name -> VarChar,
        host -> VarChar,
        port -> Integer,
    }
}