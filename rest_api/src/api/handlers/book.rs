use crate::api::response::book::BookCreateResponse;
use crate::api::response::error::AppError;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::{debug_handler, Extension, Json};
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;
use entity::book::BookCreateRequest;

#[debug_handler]
pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<BookCreateRequest>,
) -> Result<Json<BookCreateResponse>, AppError> {
    let book_active_model = payload.into_active_model();
    let book_model = book_active_model.save(state.db_conn.load().as_ref()).await?;
    let book = book_model.try_into_model()?;

    let response = BookCreateResponse {
        status: "success".to_string(),
        data: Some(book),
    };

    Ok(Json(response))
}