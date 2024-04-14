use serde::Serialize;
use entity::book::Model;
use crate::api::response::error::Status;

#[derive(Serialize)]
pub struct BookCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}

#[derive(Serialize)]
pub struct BookListResponse {
    pub status: Status,
    pub data: Vec<Model>,
}

#[derive(Serialize)]
pub struct BookGetResponse {
    pub status: Status,
    pub data: Option<Model>,
}