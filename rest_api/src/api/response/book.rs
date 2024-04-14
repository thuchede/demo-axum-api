use serde::Serialize;
use entity::book::Model;

#[derive(Serialize)]
pub struct BookCreateResponse {
    pub status: String,
    pub data: Option<Model>,
}