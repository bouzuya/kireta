use axum::{routing, Router};

async fn handler() -> &'static str {
    "Hello, World!"
}

pub fn route<T: Clone + Send + Sync + 'static>() -> Router<T> {
    Router::new().route("/", routing::get(handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{send_request, ResponseExt};
    use axum::http::{Request, StatusCode};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let app = route();
        let request = Request::builder()
            .method("GET")
            .uri("/")
            .header("Content-Type", "application/json")
            .body(hyper::Body::empty())?;
        let response = send_request(app, request).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body_as_string().await?, "Hello, World!");
        Ok(())
    }
}
