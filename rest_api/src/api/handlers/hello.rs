use axum::http::StatusCode;

pub async fn hello() -> Result<String, StatusCode> {
    Ok("\nHello world!\n\n".to_string())
}


mod tests {
    use crate::api::handlers::hello::hello;

    #[tokio::test]
    async fn test_hello() {
        assert_eq!(hello().await.unwrap(), "\nHello world!\n\n".to_string())
    }
}