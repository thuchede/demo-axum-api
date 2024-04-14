use crate::api::response::book::{BookCreateResponse, BookGetResponse, BookListResponse};
use crate::api::response::error::{AppError, Status};
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::{debug_handler, Extension, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;
use tracing::instrument;
use entity::book::Entity as Book;
use entity::book::{BookCreateRequest};
use crate::api::middleware::json::CustomJson;

#[debug_handler]
#[instrument(level = "info", name = "create_book", skip_all)]
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


#[instrument(level = "info", name = "list_books", skip_all)]
pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<BookListResponse>, AppError> {
    let dogs = Book::find().all(state.db_conn.load().as_ref()).await?;
    let n = dogs.len();

    let response = BookListResponse {
        status: Status::Success,
        data: dogs,
    };

    tracing::info!("number of books: {}", n);

    Ok(Json(response))
}


#[instrument(level = "info", name = "get_book", skip_all)]
pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(book_id): Path<i32>,
) -> Result<Json<BookGetResponse>, AppError> {
    let book = Book::find_by_id(book_id)
        .one(state.db_conn.load().as_ref())
        .await?;

    let response = BookGetResponse {
        status: Status::Success,
        data: book,
    };

    Ok(Json(response))
}