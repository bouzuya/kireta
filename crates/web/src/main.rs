mod handler;
mod infra;
mod model;
mod mutation;
mod query;
#[cfg(test)]
mod test_utils;

use axum::Server;
use handler::route;

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
    use serde_json::json;

    use crate::test_utils::{request, send_request, ResponseExt, StatusCode};

    use super::*;

    macro_rules! test_query3 {
        ($q:tt, $e:tt) => {
            test_query2(json!($q), json!($e)).await
        };
    }

    #[tokio::test]
    async fn test_bearer() -> anyhow::Result<()> {
        let query = r#"{"query":"query { bearer }"}"#;
        let expected = r#"{"data":{"bearer":"bearer1"}}"#;
        let app = route();
        let method = "POST";
        let uri = "/graphql";
        let body = query;
        let body: axum::body::Body = body.into();
        let request = axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .header(axum::http::header::AUTHORIZATION, "Bearer bearer1")
            .body(body)?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body_as_string().await?, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_hello() -> anyhow::Result<()> {
        test_query(
            r#"{"query":"query { hello }"}"#,
            r#"{"data":{"hello":"Hello, World!"}}"#,
        )
        .await
    }

    #[tokio::test]
    async fn test_hello_json() -> anyhow::Result<()> {
        test_query3!(
            {
                "query": "query { hello }"
            },
            {
                "data": {
                    "hello": "Hello, World!"
                }
            }
        )
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
    async fn test_add_graphql() -> anyhow::Result<()> {
        test_query3!(
            {
                "query": include_str!("../graphql/test_add.graphql"),
                "variables": {
                    "a": 1,
                    "b": 2
                }
            },
            {
                "data": {
                    "add": 3
                }
            }
        )
    }

    #[tokio::test]
    async fn test_check_list_schema() -> anyhow::Result<()> {
        test_query(
            r#"{"query":"query { __type(name: \"CheckList\") { name, description, kind } }"}"#,
            r#"{"data":{"__type":{"name":"CheckList","description":"check list","kind":"OBJECT"}}}"#,
        )
        .await?;
        test_query(
            r#"{"query":"query { __type(name: \"CheckList\") { fields { name } } }"}"#,
            r#"{"data":{"__type":{"fields":[{"name":"id"},{"name":"date"},{"name":"checkedItems"}]}}}"#,
        )
        .await?;

        test_query(
            r#"{"query":"query { __schema { queryType { name } } }"}"#,
            r#"{"data":{"__schema":{"queryType":{"name":"QueryRoot"}}}}"#,
        )
        .await?;

        // <https://graphql.org/learn/introspection/>
        // r#"{"query":"query { __type(name: \"...\") { fields { name } } }"}"#,
        Ok(())
    }

    #[tokio::test]
    async fn test_check_lists() -> anyhow::Result<()> {
        // dummy data
        test_query(
            r#"{"query":"query { checkLists { id, date } }"}"#,
            r#"{"data":{"checkLists":[{"id":"1","date":"2020-01-02"},{"id":"2","date":"2020-01-03"}]}}"#,
        )
        .await
    }

    #[tokio::test]
    async fn test_check_lists_checked_items() -> anyhow::Result<()> {
        // dummy data
        test_query(
            r#"{"query":"query { checkLists { id, checkedItems { id } } }"}"#,
            r#"{"data":{"checkLists":[{"id":"1","checkedItems":[{"id":"1"}]},{"id":"2","checkedItems":[{"id":"2"}]}]}}"#,
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

    #[tokio::test]
    async fn test_items_checked_check_lists() -> anyhow::Result<()> {
        // dummy data
        test_query3!(
            {
                "query": "query { items { checkedCheckLists { id }, id } }"
            },
            {
                "data": {
                    "items": [
                        {
                            "id": "1",
                            "checkedCheckLists": [
                                {
                                    "id": "1"
                                }
                            ]
                        },
                        {
                            "id": "2",
                            "checkedCheckLists": [
                                {
                                    "id": "2"
                                }
                            ]
                        }
                    ]
                }
            }
        )
    }

    #[tokio::test]
    async fn test_mutation_sign_in() -> anyhow::Result<()> {
        test_query3!(
            {
                "query": "mutation signIn($userId: String, $password: String) { signIn(userId: $userId, password: $password) }",
                "variables": {
                    "userId": "user1",
                    "password": "password1"
                }
            },
            {
                "data": {
                    "signIn": "user1:password1"
                }
            }
        )
    }

    #[tokio::test]
    async fn test_schema() -> anyhow::Result<()> {
        test_query(
            r#"{"query":"query { __schema { queryType { name } } }"}"#,
            r#"{"data":{"__schema":{"queryType":{"name":"QueryRoot"}}}}"#,
        )
        .await?;

        test_query(
            r#"{"query":"query { __schema { mutationType { name } } }"}"#,
            r#"{"data":{"__schema":{"mutationType":{"name":"MutationRoot"}}}}"#,
        )
        .await?;

        test_query(
            r#"{"query":"query { __schema { subscriptionType { name } } }"}"#,
            r#"{"data":{"__schema":{"subscriptionType":null}}}"#,
        )
        .await?;

        test_query(
            r#"{"query":"query { __type(name: \"QueryRoot\") { name, kind } }"}"#,
            r#"{"data":{"__type":{"name":"QueryRoot","kind":"OBJECT"}}}"#,
        )
        .await?;
        Ok(())
    }

    async fn test_query<B>(query: B, expected: &str) -> anyhow::Result<()>
    where
        B: Into<axum::body::Body>,
    {
        let app = route();
        let request = request("POST", "/graphql", query)?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.into_body_as_string().await?, expected);
        Ok(())
    }

    async fn test_query2(
        query: serde_json::Value,
        expected: serde_json::Value,
    ) -> anyhow::Result<()> {
        test_query(query.to_string(), expected.to_string().as_str()).await
    }
}
