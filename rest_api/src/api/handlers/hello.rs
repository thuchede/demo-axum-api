use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello world! Using configuration from {}\n\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("-".to_string())
    ))
}


#[allow(unused)]
mod tests {
    use std::sync::Arc;
    use axum::extract::State;
    use crate::api::handlers::hello::hello;
    use crate::settings::Settings;
    use crate::state::ApplicationState;

    #[tokio::test]
    async fn test_hello() {
        let state = State(Arc::new(ApplicationState::new(&Settings::default()).unwrap()));
        assert_eq!(hello(state).await.unwrap(), "\nHello world! Using configuration from -\n\n".to_string())
    }
}