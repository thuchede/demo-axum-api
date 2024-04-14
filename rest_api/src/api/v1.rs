use std::sync::Arc;
use super::handlers;
use axum::routing::{get, post};
use axum::{middleware, Router};
use crate::state::ApplicationState;
use crate::api::handlers::jwt::auth;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello::hello,
        handlers::login::login,
        handlers::book::create,
        handlers::book::list,
        handlers::book::get,
    ),
    components(
        schemas(
            crate::api::request::login::LoginRequest,
            crate::api::response::login::LoginResponse,
            crate::api::response::error::ErrorResponse,
            crate::api::response::error::Status,
            crate::api::response::book::BookGetResponse,
            crate::api::response::book::BookListResponse,
            crate::api::response::book::BookCreateResponse,
            entity::book::Model,
            entity::book::BookCreateRequest,
        ),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hello", description = "Hello"),
        (name = "login", description = "Login"),
        (name = "books", description = "Books"),
    ),
    servers(
        (url = "/v1", description = "Local server"),
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

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