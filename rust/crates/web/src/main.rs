mod app;
mod handler;
mod infra;
mod model;
#[cfg(test)]
mod test_utils;
mod use_case;

use app::App;
use axum::Server;
use handler::route;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = route(App::example());
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use anyhow::Context;
    use google_api_proto::google::firestore::v1::firestore_client::FirestoreClient;
    use google_api_proto::google::firestore::v1::precondition::ConditionType;
    use google_api_proto::google::firestore::v1::ListDocumentsRequest;
    use google_api_proto::google::firestore::v1::{
        value::ValueType, CreateDocumentRequest, DeleteDocumentRequest, Document,
        GetDocumentRequest, Precondition, UpdateDocumentRequest, Value,
    };
    use serde_json::json;

    use crate::test_utils::{request, send_request, ResponseExt, StatusCode};

    use super::*;

    macro_rules! test_query3 {
        ($q:tt, $e:tt) => {
            test_query2(json!($q), json!($e)).await
        };
    }

    macro_rules! test {
        ($a:expr, $q:tt, $e:tt) => {
            test($a, json!($q), json!($e)).await
        };
    }

    #[tokio::test]
    async fn test_firebase() -> anyhow::Result<()> {
        let project_id = "demo-project1";
        let credentials = google_authz::Credentials::builder()
            .no_credentials()
            .build()
            .await?;
        let channel = tonic::transport::Channel::from_static("http://firebase:8080")
            .connect()
            .await?;
        let channel = google_authz::GoogleAuthz::builder(channel)
            .credentials(credentials)
            .build()
            .await;

        let database_id = "(default)";
        let collection_name = "users";

        let mut client = FirestoreClient::new(channel);

        // reset
        let response = client
            .list_documents(tonic::Request::new(ListDocumentsRequest {
                parent: format!(
                    "projects/{}/databases/{}/documents",
                    project_id, database_id
                ),
                collection_id: collection_name.to_owned(),
                page_size: 100,
                ..Default::default()
            }))
            .await?;
        let list = response.into_inner();
        for doc in list.documents {
            client
                .delete_document(tonic::Request::new(DeleteDocumentRequest {
                    name: doc.name,
                    current_document: None,
                }))
                .await?;
        }

        // CREATE
        let response = client
            .create_document(tonic::Request::new(CreateDocumentRequest {
                parent: format!(
                    "projects/{}/databases/{}/documents",
                    project_id, database_id
                ),
                collection_id: collection_name.to_owned(),
                document_id: "".to_owned(),
                document: Some(Document {
                    name: "".to_owned(),
                    fields: {
                        let mut fields = BTreeMap::new();
                        fields.insert(
                            "k1".to_owned(),
                            Value {
                                value_type: Some(ValueType::StringValue("v1".to_owned())),
                            },
                        );
                        fields
                    },
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            }))
            .await?;
        let created = response.into_inner();
        assert!(created
            .name
            .starts_with("projects/demo-project1/databases/(default)/documents/users/"),);
        assert_eq!(created.fields, {
            let mut fields = BTreeMap::new();
            fields.insert(
                "k1".to_owned(),
                Value {
                    value_type: Some(ValueType::StringValue("v1".to_owned())),
                },
            );
            fields
        });
        assert!(created.create_time.is_some());
        assert!(created.update_time.is_some());

        // READ (GET)
        let response = client
            .get_document(tonic::Request::new(GetDocumentRequest {
                name: created.name.clone(),
                mask: None,
                consistency_selector: None,
            }))
            .await?;
        let got = response.into_inner();
        assert_eq!(got, created);

        // READ (LIST)
        let response = client
            .list_documents(tonic::Request::new(ListDocumentsRequest {
                parent: format!(
                    "projects/{}/databases/{}/documents",
                    project_id, database_id
                ),
                collection_id: collection_name.to_owned(),
                page_size: 100,
                ..Default::default()
            }))
            .await?;
        let list = response.into_inner();
        assert_eq!(list.documents, vec![got.clone()]);
        assert_eq!(list.next_page_token, "");

        // UPDATE
        let response = client
            .update_document(tonic::Request::new(UpdateDocumentRequest {
                document: Some(Document {
                    fields: {
                        let mut fields = BTreeMap::new();
                        fields.insert(
                            "k1".to_owned(),
                            Value {
                                value_type: Some(ValueType::StringValue("v2".to_owned())),
                            },
                        );
                        fields
                    },
                    ..got.clone()
                }),
                update_mask: None,
                mask: None,
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::UpdateTime(
                        got.update_time.context("update_time")?,
                    )),
                }),
            }))
            .await?;
        let updated = response.into_inner();
        assert_eq!(
            updated.fields.get("k1"),
            Some(&Value {
                value_type: Some(ValueType::StringValue("v2".to_owned()))
            })
        );

        // DELETE
        client
            .delete_document(tonic::Request::new(DeleteDocumentRequest {
                name: updated.name,
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::UpdateTime(
                        updated.update_time.context("update_time")?,
                    )),
                }),
            }))
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_bearer() -> anyhow::Result<()> {
        let query = r#"{"query":"query { bearer }"}"#;
        let expected = r#"{"data":{"bearer":"bearer1"}}"#;
        let app = route(App::example());
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
        test!(
            App::example(),
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
        test!(
            App::example(),
            {
                "query": "query { add(a: 1, b: 2) }"
            },
            {
                "data": {
                    "add": 3
                }
            }
        )
    }

    #[tokio::test]
    async fn test_add_graphql() -> anyhow::Result<()> {
        test!(
            App::example(),
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
        test!(
            App::example(),
            {
                "query": r#"query { __type(name: "CheckList") { description, kind, name } }"#
            },
            {
                "data":{
                    "__type":{
                        "description": "check list",
                        "kind": "OBJECT",
                        "name": "CheckList",
                    }
                }
            }
        )?;
        test!(
            App::example(),
            {
                "query": r#"query { __type(name: "CheckList") { fields { name } } }"#
            },
            {
                "data": {
                    "__type": {
                        "fields": [
                            { "name": "id" },
                            { "name": "date" },
                            { "name": "checkedItems" }
                        ]
                    }
                }
            }
        )?;
        test!(
            App::example(),
            {
                "query": "query { __schema { queryType { name } } }"
            },
            {
                "data": {
                    "__schema": {
                        "queryType": {
                            "name": "QueryRoot"
                        }
                    }
                }
            }
        )?;

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
        let app = route(App::example());
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

    async fn test(
        app: App,
        request_body: serde_json::Value,
        response_body: serde_json::Value,
    ) -> anyhow::Result<()> {
        let app = route(app);
        let request = request("POST", "/graphql", request_body.to_string())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.into_body_as_string().await?,
            response_body.to_string()
        );
        Ok(())
    }
}
