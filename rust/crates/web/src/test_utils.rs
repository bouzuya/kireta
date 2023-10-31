pub use axum::http::Method;
pub use axum::http::StatusCode;

#[axum::async_trait]
pub trait ResponseExt {
    async fn into_body_as_string(self) -> anyhow::Result<String>;
}

#[axum::async_trait]
impl ResponseExt for axum::response::Response<axum::body::BoxBody> {
    async fn into_body_as_string(self) -> anyhow::Result<String> {
        let body = self.into_body();
        let bytes = hyper::body::to_bytes(body).await?.to_vec();
        let s = String::from_utf8(bytes)?;
        Ok(s)
    }
}

pub fn request<B>(
    method: &str,
    uri: &str,
    body: B,
) -> Result<axum::http::Request<axum::body::Body>, axum::http::Error>
where
    B: std::convert::Into<axum::body::Body>,
{
    let body: axum::body::Body = body.into();
    axum::http::Request::builder()
        .method(method)
        .uri(uri)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(body)
}

pub async fn send_request(
    app: axum::Router,
    request: axum::http::Request<axum::body::Body>,
) -> Result<axum::response::Response<axum::body::BoxBody>, std::convert::Infallible> {
    tower::ServiceExt::oneshot(app, request).await
}
