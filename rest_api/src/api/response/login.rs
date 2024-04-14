use serde::Serialize;
use crate::api::response::error::Status;

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}