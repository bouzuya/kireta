use axum::{routing, Router, Server};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/", routing::get(|| async { "Hello, World!" }));
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
