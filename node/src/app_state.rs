use std::{sync::Arc, error::Error, fmt::Display};
use crate::{config::Config, data_store::postgres::{PostgresDataStore, CreatePostgresDataStoreError}};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_store: Arc<PostgresDataStore>,
    pub node_store: Arc<PostgresDataStore>,
}

#[derive(Debug)]
pub enum CreateAppStateError {
    PostgresDataStore(CreatePostgresDataStoreError),
}

impl Error for CreateAppStateError { }

impl Display for CreateAppStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostgresDataStore(p) => p.fmt(f),
        }
    }
}

impl From<CreatePostgresDataStoreError> for CreateAppStateError {
    fn from(value: CreatePostgresDataStoreError) -> Self {
        Self::PostgresDataStore(value)
    }
}

impl TryFrom<Config> for AppState {
    type Error = CreateAppStateError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let data_store = PostgresDataStore::try_from(&value)?;

        let data_store_arc = Arc::new(data_store);

        Ok(Self {
            user_store: data_store_arc.clone(),
            node_store: data_store_arc.clone(),
            config: Arc::new(value),
        })
    }
}