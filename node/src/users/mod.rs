pub mod store;
pub mod web;

pub struct User {
    id: i32,
    #[allow(dead_code)]
    username: String,
}