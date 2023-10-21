mod model;
mod query;
#[cfg(test)]
mod test_utils;

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

fn route() -> Router {
    let schema = Schema::build(query::QueryRoot, EmptyMutation, EmptySubscription)
        .data(Store::example())
        .finish();
    Router::new()
        .route(
            "/graphql",
            routing::get(graphiql).post_service(GraphQL::new(schema)),
        )
        .route("/", routing::get(|| async { "Hello, World!" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = route();
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{request, send_request, ResponseExt, StatusCode};

    use super::*;

    #[tokio::test]
    async fn test_hello() -> anyhow::Result<()> {
        test_query(
            r#"{"query":"query { hello }"}"#,
            r#"{"data":{"hello":"Hello, World!"}}"#,
        )
        .await
    }

    #[tokio::test]
    async fn test_add() -> anyhow::Result<()> {
        test_query(
            r#"{"query":"query { add(a: 1, b: 2) }"}"#,
            r#"{"data":{"add":3}}"#,
        )
        .await
    }

    #[tokio::test]
    async fn test_check_lists() -> anyhow::Result<()> {
        // dummy data
        test_query(
            r#"{"query":"query { checkLists { id, date } }"}"#,
            r#"{"data":{"checkLists":[{"id":"1","date":"2020-01-02"}]}}"#,
        )
        .await
    }

    #[tokio::test]
    async fn test_check_lists_checked_items() -> anyhow::Result<()> {
        // dummy data
        test_query(
            r#"{"query":"query { checkLists { id, checkedItems { id } } }"}"#,
            r#"{"data":{"checkLists":[{"id":"1","checkedItems":[{"id":"1"}]}]}}"#,
        )
        .await
    }

    #[tokio::test]
    async fn test_items() -> anyhow::Result<()> {
        // dummy data
        test_query(
            r#"{"query":"query { items { id, name } }"}"#,
            r#"{"data":{"items":[{"id":"1","name":"item1"},{"id":"2","name":"item2"}]}}"#,
        )
        .await
    }

    async fn test_query(query: &'static str, expected: &str) -> anyhow::Result<()> {
        let app = route();
        let request = request("POST", "/graphql", query)?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body_as_string().await?, expected);
        Ok(())
    }
}
