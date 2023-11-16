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

pub fn route<T: Clone + HasSchema + HasStore + Send + Sync + 'static>() -> Router<T> {
    Router::new().route("/graphql", routing::get(graphiql).post(handler::<T>))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::App,
        test_utils::{send_request, ResponseExt},
    };
    use axum::http::{Request, StatusCode};

    #[tokio::test]
    async fn test_get() -> anyhow::Result<()> {
        let app = route().with_state(App::example());
        let request = Request::builder()
            .method("GET")
            .uri("/graphql")
            .body(hyper::Body::empty())?;
        let response = send_request(app, request).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response
            .into_body_as_string()
            .await?
            .contains("<title>GraphiQL IDE</title>"));
        Ok(())
    }

    #[tokio::test]
    async fn test_post() -> anyhow::Result<()> {
        let app = route().with_state(App::example());
        let request = Request::builder()
            .method("POST")
            .uri("/graphql")
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::Body::from(r#"{"query":"query test { hello }"}"#))?;
        let response = send_request(app, request).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.into_body_as_string().await?,
            r#"{"data":{"hello":"Hello, World!"}}"#
        );
        Ok(())
    }
}
