mod model;
mod query;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing, Router, Server,
};
use query::Store;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = Schema::build(query::QueryRoot, EmptyMutation, EmptySubscription)
        .data(Store::example())
        .finish();
    let app = Router::new()
        .route(
            "/graphql",
            routing::get(graphiql).post_service(GraphQL::new(schema)),
        )
        .route("/", routing::get(|| async { "Hello, World!" }));
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
