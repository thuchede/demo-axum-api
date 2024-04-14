use serde::Serialize;
use entity::book::Model;
use crate::api::response::error::Status;

#[derive(Serialize)]
pub struct BookCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}