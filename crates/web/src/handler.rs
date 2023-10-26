use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
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

use crate::{infra::store::Store, query};

async fn handler(
    State(schema): State<Schema<query::QueryRoot, EmptyMutation, EmptySubscription>>,
    header_map: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    // TODO
    let header_value = header_map.get(axum::http::header::AUTHORIZATION).unwrap();
    // TODO:
    let bearer = Bearer::decode(header_value).unwrap();
    GraphQLResponse::from(schema.execute(request.into_inner().data(bearer)).await)
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn route() -> Router {
    let schema = Schema::build(query::QueryRoot, EmptyMutation, EmptySubscription)
        .data(Store::example())
        .finish();
    Router::new()
        .route("/graphql", routing::get(graphiql).post(handler))
        .route("/", routing::get(|| async { "Hello, World!" }))
        .with_state(schema)
}
