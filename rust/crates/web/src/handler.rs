use async_graphql::http::GraphiQLSource;
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
use hyper::StatusCode;

use crate::use_case::{HasSchema, HasStore, Store};

async fn handler<T: HasSchema + HasStore>(
    State(state): State<T>,
    header_map: HeaderMap,
    request: GraphQLRequest,
) -> Result<GraphQLResponse, StatusCode> {
    let schema = state.schema();
    let store = state.store();
    let request = request.into_inner().data(Data(Box::new(store)));
    let request = if let Some(header_value) = header_map.get(axum::http::header::AUTHORIZATION) {
        let bearer = Bearer::decode(header_value).ok_or(StatusCode::UNAUTHORIZED)?;
        request.data(bearer)
    } else {
        request
    };
    Ok(GraphQLResponse::from(schema.execute(request).await))
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub struct Data(pub Box<dyn Store + Send + Sync + 'static>);

pub fn route<T: Clone + HasSchema + HasStore + Send + Sync + 'static>(store: T) -> Router {
    Router::new()
        .route("/graphql", routing::get(graphiql).post(handler::<T>))
        .route("/", routing::get(|| async { "Hello, World!" }))
        .with_state(store)
}
