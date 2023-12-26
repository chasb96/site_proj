pub mod store;
pub mod web;

pub struct FileMetadata {
    pub id: i32,
    pub name: String,
    pub content_type: String,
    pub extension: String,
    pub internal_name: String,
}

pub struct NewFileMetadata {
    pub name: String,
    pub content_type: String,
    pub extension: Option<String>,
    pub internal_name: String,
}