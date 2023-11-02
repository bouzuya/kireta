use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, precondition::ConditionType, value::ValueType,
    CreateDocumentRequest, DeleteDocumentRequest, Document, GetDocumentRequest,
    ListDocumentsRequest, ListDocumentsResponse, MapValue, Precondition, UpdateDocumentRequest,
};
use google_authz::{Credentials, GoogleAuthz};
use prost_types::Timestamp;
use serde::Serialize;
use serde_firestore_value::to_value;
use tonic::{transport::Channel, Request};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("credentials {0}")]
    Credentials(#[from] google_authz::CredentialsError),
    #[error("serialize {0}")]
    Serialize(#[from] serde_firestore_value::Error),
    #[error("status {0}")]
    Status(#[from] tonic::Status),
    #[error("transport {0}")]
    Transport(#[from] tonic::transport::Error),
}

pub struct Client {
    client: FirestoreClient<GoogleAuthz<Channel>>,
    database_id: String,
    project_id: String,
}

impl Client {
    // TODO: begin_transaction
    // TODO: commit
    // TODO: rollback

    pub async fn new(
        project_id: String,
        database_id: String,
        endpoint: &'static str,
    ) -> Result<Self, Error> {
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

    pub async fn create<V>(&mut self, collection_id: String, fields: V) -> Result<Document, Error>
    where
        V: Serialize,
    {
        let response = self
            .client
            .create_document(Request::new(CreateDocumentRequest {
                parent: format!(
                    "projects/{}/databases/{}/documents",
                    self.project_id, self.database_id
                ),
                collection_id,
                document_id: "".to_string(),
                document: Some(Document {
                    name: "".to_string(),
                    fields: {
                        let ser = to_value(&fields)?;
                        if let Some(ValueType::MapValue(MapValue { fields })) = ser.value_type {
                            fields
                        } else {
                            panic!("unexpected value_type: {:?}", ser.value_type);
                        }
                    },
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            }))
            .await?;
        Ok(response.into_inner())
    }

    pub async fn delete(
        &mut self,
        // `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        name: String,
        current_update_time: Timestamp,
    ) -> Result<(), Error> {
        self.client
            .delete_document(Request::new(DeleteDocumentRequest {
                name,
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::UpdateTime(current_update_time)),
                }),
            }))
            .await?;
        Ok(())
    }

    pub async fn get(
        &mut self,
        // `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        name: String,
        // TODO: support transaction
    ) -> Result<Document, Error> {
        let response = self
            .client
            .get_document(Request::new(GetDocumentRequest {
                name,
                mask: None,
                consistency_selector: None,
            }))
            .await?;
        Ok(response.into_inner())
    }

    pub async fn list(
        &mut self,
        collection_id: String, // TODO: support some params
    ) -> Result<ListDocumentsResponse, Error> {
        let response = self
            .client
            .list_documents(Request::new(ListDocumentsRequest {
                parent: format!(
                    "projects/{}/databases/{}/documents",
                    self.project_id, self.database_id
                ),
                collection_id,
                page_size: 100,
                ..Default::default()
            }))
            .await?;
        Ok(response.into_inner())
    }

    pub async fn update<V>(
        &mut self,
        name: String,
        fields: V,
        current_update_time: Timestamp,
    ) -> Result<Document, Error>
    where
        V: Serialize,
    {
        let response = self
            .client
            .update_document(Request::new(UpdateDocumentRequest {
                document: Some(Document {
                    name,
                    fields: {
                        let ser = to_value(&fields)?;
                        if let Some(ValueType::MapValue(MapValue { fields })) = ser.value_type {
                            fields
                        } else {
                            panic!("unexpected value_type: {:?}", ser.value_type);
                        }
                    },
                    create_time: None,
                    update_time: None,
                }),
                update_mask: None,
                mask: None,
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::UpdateTime(current_update_time)),
                }),
            }))
            .await?;
        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use anyhow::Context;
    use google_api_proto::google::firestore::v1::{value::ValueType, Value};

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let project_id = "demo-project1";
        let database_id = "(default)";
        let collection_name = "repositories";

        let mut client =
            Client::new(project_id.to_string(), database_id.to_string(), endpoint).await?;

        // reset
        let list = client.list(collection_name.to_owned()).await?;
        for doc in list.documents {
            client
                .delete(doc.name, doc.update_time.context("update_time")?)
                .await?;
        }

        // CREATE
        #[derive(serde::Serialize)]
        struct V {
            k1: String,
        }
        let created = client
            .create(
                collection_name.to_string(),
                V {
                    k1: "v1".to_owned(),
                },
            )
            .await?;
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
        let got = client.get(created.name.clone()).await?;
        assert_eq!(got, created);

        // READ (LIST)
        let list = client.list(collection_name.to_owned()).await?;
        assert_eq!(list.documents, vec![got.clone()]);
        assert_eq!(list.next_page_token, "");

        // UPDATE
        let updated = client
            .update(
                got.name.clone(),
                V {
                    k1: "v2".to_owned(), // "v1" -> "v2
                },
                got.update_time.context("update_time")?,
            )
            .await?;
        assert_eq!(
            updated.fields.get("k1"),
            Some(&Value {
                value_type: Some(ValueType::StringValue("v2".to_owned()))
            })
        );

        // DELETE
        client
            .delete(updated.name, updated.update_time.context("update_time")?)
            .await?;

        Ok(())
    }
}
