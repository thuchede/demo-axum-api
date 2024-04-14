use std::sync::Arc;
use super::handlers;
use axum::routing::{get, post};
use axum::{middleware, Router};
use crate::state::ApplicationState;
use crate::api::handlers::jwt::auth;


pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/hello", get(handlers::hello::hello).with_state(state.clone()))
        .route("/login", post(handlers::login::login).with_state(state.clone()))
        .route(
            "/books",
            post(handlers::book::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route("/books", get(handlers::book::list).with_state(state.clone()))
        .route("/books/:id", get(handlers::book::get).with_state(state))
}