use std::sync::Arc;

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
    let request = request.into_inner().data(Data {
        bearer: match header_map.get(axum::http::header::AUTHORIZATION) {
            Some(header_value) => {
                Some(Bearer::decode(header_value).ok_or(StatusCode::UNAUTHORIZED)?)
            }
            None => None,
        },
        store,
    });
    Ok(GraphQLResponse::from(schema.execute(request).await))
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub struct Data {
    pub bearer: Option<Bearer>,
    pub store: Arc<dyn Store + Send + Sync>,
}

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

    #[tokio::test]
    async fn test_get() -> anyhow::Result<()> {
        let app = route().with_state(App::example());
        let request = axum::http::Request::builder()
            .method("GET")
            .uri("/graphql")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert!(response
            .into_body_as_string()
            .await?
            .contains("<title>GraphiQL IDE</title>"));
        Ok(())
    }

    #[tokio::test]
    async fn test_post() -> anyhow::Result<()> {
        let app = route().with_state(App::example());
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/graphql")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                r#"{"query":"query test_post { hello }"}"#,
            ))?;
        let response = send_request(app, request).await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_as_string().await?,
            r#"{"data":{"hello":"Hello, World!"}}"#
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_bearer() -> anyhow::Result<()> {
        let app = route().with_state(App::example());
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/graphql")
            .header(axum::http::header::AUTHORIZATION, "Bearer test")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                r#"{"query":"query test_post_bearer { bearer }"}"#,
            ))?;
        let response = send_request(app, request).await?;

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_as_string().await?,
            r#"{"data":{"bearer":"test"}}"#
        );
        Ok(())
    }
}
