use std::sync::Arc;
use crate::{config::Config, users::store::UserStoreProvider, data_store::postgres::PostgresDataStore};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_store: Arc<PostgresDataStore>,
}

impl From<Config> for AppState {
    fn from(value: Config) -> Self {
        let data_store: PostgresDataStore = UserStoreProvider::from_config(&value);

        Self {
            user_store: Arc::new(data_store),
            config: Arc::new(value),
        }
    }
}