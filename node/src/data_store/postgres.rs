use std::{error::Error, fmt::Display};

use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;
use crate::config::Config;

pub struct PostgresDataStore {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Debug)]
pub enum CreatePostgresDataStoreError {
    Pool(r2d2::Error)
}

impl Error for CreatePostgresDataStoreError { }

impl Display for CreatePostgresDataStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pool(p) => p.fmt(f),
        }
    }
}

impl From<r2d2::Error> for CreatePostgresDataStoreError {
    fn from(value: r2d2::Error) -> Self {
        Self::Pool(value)
    }
}

impl TryFrom<&Config> for PostgresDataStore {
    type Error = CreatePostgresDataStoreError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        let connection_string = value
            .database
            .connection_string
            .to_string();

        let manager = ConnectionManager::<PgConnection>::new(connection_string);

        let pool = Pool::builder()
            .build(manager)?;

        Ok(Self {
            connection_pool: pool
        })
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