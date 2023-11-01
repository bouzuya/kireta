use std::collections::BTreeMap;

use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, CreateDocumentRequest, Document, Value,
};
use google_authz::{Credentials, GoogleAuthz};
use tonic::{transport::Channel, Request, Status};

pub struct Client {
    client: FirestoreClient<GoogleAuthz<Channel>>,
    database_id: String,
    project_id: String,
}

impl Client {
    pub async fn new(
        project_id: String,
        database_id: String,
        endpoint: &'static str,
    ) -> anyhow::Result<Self> {
        let credentials = Credentials::builder().no_credentials().build().await?;
        let channel = Channel::from_static(endpoint).connect().await?;
        let channel = GoogleAuthz::builder(channel)
            .credentials(credentials)
            .build()
            .await;
        let client = FirestoreClient::new(channel);
        Ok(Self {
            client,
            database_id,
            project_id,
        })
    }

    pub async fn create(
        &mut self,
        collection_id: String,
        fields: BTreeMap<String, Value>,
    ) -> Result<Document, Status> {
        self.client
            .create_document(Request::new(CreateDocumentRequest {
                parent: format!(
                    "projects/{}/databases/{}/documents",
                    self.project_id, self.database_id
                ),
                collection_id,
                document_id: "".to_string(),
                document: Some(Document {
                    name: "".to_string(),
                    fields,
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            }))
            .await
            .map(|response| response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use google_api_proto::google::firestore::v1::{
        precondition::ConditionType, value::ValueType, DeleteDocumentRequest, GetDocumentRequest,
        ListDocumentsRequest, Precondition, UpdateDocumentRequest,
    };

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let project_id = "demo-project1";
        let credentials = google_authz::Credentials::builder()
            .no_credentials()
            .build()
            .await?;
        let channel = tonic::transport::Channel::from_static(endpoint)
            .connect()
            .await?;
        let channel = google_authz::GoogleAuthz::builder(channel)
            .credentials(credentials)
            .build()
            .await;

        let database_id = "(default)";
        let collection_name = "repositories";

        let mut client = FirestoreClient::new(channel);
        let mut client2 =
            Client::new(project_id.to_string(), database_id.to_string(), endpoint).await?;

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
        let mut fields = BTreeMap::new();
        fields.insert(
            "k1".to_owned(),
            Value {
                value_type: Some(ValueType::StringValue("v1".to_owned())),
            },
        );
        let created = client2.create(collection_name.to_string(), fields).await?;
        assert!(created
            .name
            .starts_with("projects/demo-project1/databases/(default)/documents/repositories/"),);
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
}
