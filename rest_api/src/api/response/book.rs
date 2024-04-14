use serde::Serialize;
use utoipa::ToSchema;
use entity::book::Model;
use crate::api::response::error::Status;

#[derive(Serialize, ToSchema)]
pub struct BookCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}

#[derive(Serialize, ToSchema)]
pub struct BookListResponse {
    pub status: Status,
    pub data: Vec<Model>,
}

#[derive(Serialize, ToSchema)]
pub struct BookGetResponse {
    pub status: Status,
    pub data: Option<Model>,
}