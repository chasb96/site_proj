use std::{error::Error, fmt::Display};
use crate::{data_store::files::{InitializeFileStoreError, initialize_file_store}, config::Config};

#[derive(Debug)]
pub enum FileDataStoreStartupError {
    Initialize(InitializeFileStoreError)
}

impl Error for FileDataStoreStartupError { }

impl Display for FileDataStoreStartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileDataStoreStartupError::Initialize(e) => e.fmt(f),
        }
    }
}

impl From<InitializeFileStoreError> for FileDataStoreStartupError {
    fn from(value: InitializeFileStoreError) -> Self {
        Self::Initialize(value)
    }
}

pub fn file_store_start(config: &Config) -> Result<(), FileDataStoreStartupError> {
    initialize_file_store(&config.file_store)
        .map_err(Into::into)
}