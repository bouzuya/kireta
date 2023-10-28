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

use crate::{
    mutation, query,
    use_case::{HasSchema, StoreTrait},
};

async fn handler<T: HasSchema>(
    State(state): State<T>,
    header_map: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let schema = state.schema();
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

pub struct Data(pub Box<dyn StoreTrait + Send + Sync + 'static>);

#[derive(Clone)]
pub struct MyState {
    schema: Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>,
}

impl HasSchema for MyState {
    fn schema(&self) -> Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription> {
        self.schema.clone()
    }
}

pub fn route<T: StoreTrait + Send + Sync + 'static>(store: T) -> Router {
    let schema = Schema::build(query::QueryRoot, mutation::MutationRoot, EmptySubscription)
        .data(Data(Box::new(store)))
        .finish();
    let state = MyState { schema };
    Router::new()
        .route("/graphql", routing::get(graphiql).post(handler::<MyState>))
        .route("/", routing::get(|| async { "Hello, World!" }))
        .with_state(state)
}
