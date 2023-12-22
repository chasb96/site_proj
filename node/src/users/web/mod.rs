use axum::{extract::FromRequestParts, async_trait, http::{request::Parts, StatusCode}};

use crate::data_store::postgres::PostgresDataStore;
use super::store::UserStore;

mod request;
mod response;
pub mod routes;

pub struct UserStoreExtractor(PostgresDataStore);

impl UserStore for UserStoreExtractor {
    async fn create(&self, username: String, password_hash: String) -> Result<super::User, super::store::error::CreateUserError> {
        self.0.create(username, password_hash).await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<super::User>, super::store::error::GetUserError> {
        self.0.get_by_id(id).await
    }

    async fn get_by_username<'a>(&self, username: &'a str) -> Result<Option<super::User>, super::store::error::GetUserError> {
        self.0.get_by_username(username).await
    }

    async fn delete(&self, id: i32) -> Result<bool, super::store::error::DeleteUserError> {
        self.0.delete(id).await
    }

    async fn get_password_hash(&self, id: i32) -> Result<Option<String>, super::store::error::GetPasswordError> {
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