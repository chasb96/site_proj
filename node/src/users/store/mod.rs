use crate::{data_store::postgres::PostgresDataStore, config::Config};
use self::error::{CreateUserError, GetUserError, DeleteUserError};

use super::User;

mod postgres;
mod error;

pub trait UserStore {
    async fn create(&self, username: String) -> Result<User, CreateUserError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetUserError>;

    async fn get_by_username(&self, username: String) -> Result<Option<User>, GetUserError>;

    async fn delete(&self, id: i32) -> Result<bool, DeleteUserError>;
}

pub struct UserStoreProvider;

impl UserStoreProvider {
    pub fn from_config(config: &Config) -> PostgresDataStore {
        PostgresDataStore::from(config)
    }
}