use url::Host;

pub mod store;
pub mod web;

pub struct Node {
    id: i32,
    name: String,
    host: Host,
    port: u16,
}