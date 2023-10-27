use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    headers::{
        authorization::{Bearer, Credentials},
        HeaderMap,
    },
    response::{Html, IntoResponse},
    routing, Router,
};

use crate::{infra::store::Store, mutation, query};

async fn handler(
    State(schema): State<Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>>,
    header_map: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let req = if let Some(header_value) = header_map.get(axum::http::header::AUTHORIZATION) {
        let bearer = Bearer::decode(header_value).unwrap();
        request.into_inner().data(bearer)
    } else {
        request.into_inner()
    };
    GraphQLResponse::from(schema.execute(req).await)
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn route() -> Router {
    let schema = Schema::build(query::QueryRoot, mutation::MutationRoot, EmptySubscription)
        .data(Store::example())
        .finish();
    Router::new()
        .route("/graphql", routing::get(graphiql).post(handler))
        .route("/", routing::get(|| async { "Hello, World!" }))
        .with_state(schema)
}
