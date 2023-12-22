use axum::{extract::FromRequestParts, async_trait, http::{request::Parts, StatusCode}};
use crate::{data_store::postgres::PostgresDataStore, users::{store::{UserStore, error::{DeleteUserError, GetPasswordError, GetUserError, CreateUserError}}, User}};

pub struct UserStoreExtractor(PostgresDataStore);

impl UserStore for UserStoreExtractor {
    async fn create(&self, username: String, password_hash: String) -> Result<User, CreateUserError> {
        self.0.create(username, password_hash).await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetUserError> {
        self.0.get_by_id(id).await
    }

    async fn get_by_username<'a>(&self, username: &'a str) -> Result<Option<User>, GetUserError> {
        self.0.get_by_username(username).await
    }

    async fn delete(&self, id: i32) -> Result<bool, DeleteUserError> {
        self.0.delete(id).await
    }

    async fn get_password_hash(&self, id: i32) -> Result<Option<String>, GetPasswordError> {
        self.0.get_password_hash(id).await
    }
}

impl Default for UserStoreExtractor {
    fn default() -> Self {
        UserStoreExtractor(PostgresDataStore::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for UserStoreExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) ->  Result<Self,Self::Rejection> {
        Ok(UserStoreExtractor::default())
    }
}