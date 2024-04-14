use crate::api::response::book::BookCreateResponse;
use crate::api::response::error::{AppError, Status};
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::{debug_handler, Extension, Json};
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;
use entity::book::BookCreateRequest;
use crate::api::middleware::json::CustomJson;

#[debug_handler]
pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<BookCreateRequest>,
) -> Result<Json<BookCreateResponse>, AppError> {
    let book_active_model = payload.into_active_model();
    let book_model = book_active_model.save(state.db_conn.load().as_ref()).await?;
    let book = book_model.try_into_model()?;

    let response = BookCreateResponse {
        status: Status::Success,
        data: Some(book),
    };

    Ok(Json(response))
}