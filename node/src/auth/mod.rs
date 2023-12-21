pub mod web;
pub mod password;

use axum::{extract::FromRequest, http::{Request, StatusCode}, body::Body, async_trait};
use crate::users::User;

#[async_trait]
impl<T> FromRequest<T> for User {
    type Rejection = StatusCode;

    async fn from_request<'a>(req: Request<Body>, state: &'a T) ->  Result<Self,Self::Rejection> {
        todo!()
    }
}