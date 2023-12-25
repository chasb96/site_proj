use std::{sync::OnceLock, path::Path};
use std::{error::Error, fmt::Display};
use crate::config::FileStoreConfig;

static DATA_DIRECTORY: OnceLock<String> = OnceLock::new();

#[derive(Debug)]
pub struct InitializeFileStoreError(String);

impl Error for InitializeFileStoreError { }

impl Display for InitializeFileStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path {} does not exist", self.0)
    }
}

pub fn initialize_file_store(config: &FileStoreConfig) -> Result<(), InitializeFileStoreError> {
    if !Path::new(&config.path).exists() {
        return Err(InitializeFileStoreError(config.path.clone()))
    }

    DATA_DIRECTORY
        .get_or_init(|| {
            config.path.clone()
        });

    Ok(())
}

pub struct DiskDataStore {
    pub directory: &'static str,
}

impl Default for DiskDataStore {
    fn default() -> Self {
        Self { 
            directory: DATA_DIRECTORY
                .get()
                .expect("Data directory used before initialization")
        }
    }
}