pub mod store;
pub mod web;

pub struct User {
    pub id: i32,
    pub username: String,
    pub admin: bool,
}