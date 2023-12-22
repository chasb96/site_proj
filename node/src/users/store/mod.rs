use self::error::{CreateUserError, GetUserError, DeleteUserError, GetPasswordError};
use super::User;

mod postgres;
pub mod error;

pub trait UserStore {
    async fn create(&self, username: String, password_hash: String) -> Result<User, CreateUserError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetUserError>;

    async fn get_by_username<'a>(&self, username: &'a str) -> Result<Option<User>, GetUserError>;

    async fn delete(&self, id: i32) -> Result<bool, DeleteUserError>;

    async fn get_password_hash(&self, id: i32) -> Result<Option<String>, GetPasswordError>;
}
