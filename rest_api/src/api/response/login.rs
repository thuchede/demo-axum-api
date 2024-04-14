use serde::Serialize;
use utoipa::ToSchema;
use crate::api::response::error::Status;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}