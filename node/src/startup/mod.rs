mod postgres;
mod files;

use std::{error::Error, fmt::Display};
use crate::config::Config;
use self::{postgres::PostgresStartupError, files::FileDataStoreStartupError};

#[derive(Debug)]
pub enum StartupError {
    Postgres(PostgresStartupError),
    Files(FileDataStoreStartupError),
}

impl Error for StartupError { }

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartupError::Postgres(e) => e.fmt(f),
            StartupError::Files(e) => e.fmt(f),
        }
    }
}

impl From<PostgresStartupError> for StartupError {
    fn from(value: PostgresStartupError) -> Self {
        Self::Postgres(value)
    }
}

impl From<FileDataStoreStartupError> for StartupError {
    fn from(value: FileDataStoreStartupError) -> Self {
        Self::Files(value)
    }
}

pub async fn on_start(config: &Config) -> Result<(), StartupError> {
    postgres::postgres_start(config).await?;

    files::file_store_start(config)?;

    Ok(())
}