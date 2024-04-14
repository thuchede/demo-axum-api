use std::sync::Arc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::state::ApplicationState;

mod handlers;
mod v1;
mod request;
mod response;
mod middleware;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url(
            "/v1/api-docs/openapi.json",
            v1::ApiDoc::openapi(),
        ))
        .nest("/v1", v1::configure(state))
}