mod document;
mod path;

use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, precondition::ConditionType, value::ValueType,
    CreateDocumentRequest, DeleteDocumentRequest, Document as FirestoreDocument,
    GetDocumentRequest, ListDocumentsRequest, ListDocumentsResponse, MapValue, Precondition,
    UpdateDocumentRequest,
};
use google_authz::{Credentials, GoogleAuthz};
use prost_types::Timestamp;
use serde::{de::DeserializeOwned, Serialize};
use serde_firestore_value::to_value;
use tonic::{transport::Channel, Request};

use self::document::Document;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("credentials {0}")]
    Credentials(#[from] google_authz::CredentialsError),
    #[error("deserialize {0}")]
    Deserialize(#[from] document::Error),
    #[error("serialize {0}")]
    Serialize(#[from] serde_firestore_value::Error),
    #[error("status {0}")]
    Status(#[from] tonic::Status),
    #[error("transport {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("value_type")]
    ValueType,
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

    pub async fn create<T, U>(
        &mut self,
        collection_id: String,
        fields: T,
    ) -> Result<Document<U>, Error>
    where
        T: Serialize,
        U: DeserializeOwned,
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
                document: Some(FirestoreDocument {
                    name: "".to_string(),
                    fields: {
                        let ser = to_value(&fields)?;
                        if let Some(ValueType::MapValue(MapValue { fields })) = ser.value_type {
                            fields
                        } else {
                            return Err(Error::ValueType);
                        }
                    },
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            }))
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
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

    pub async fn get<U>(
        &mut self,
        // `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        name: String,
        // TODO: support transaction
    ) -> Result<Document<U>, Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .get_document(Request::new(GetDocumentRequest {
                name,
                mask: None,
                consistency_selector: None,
            }))
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }

    pub async fn list<U>(
        &mut self,
        collection_id: String, // TODO: support some params
    ) -> Result<(Vec<Document<U>>, String), Error>
    where
        U: DeserializeOwned,
    {
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
        let ListDocumentsResponse {
            documents,
            next_page_token,
        } = response.into_inner();
        documents
            .into_iter()
            .map(Document::new)
            .collect::<Result<Vec<Document<U>>, document::Error>>()
            .map_err(Error::Deserialize)
            .map(|documents| (documents, next_page_token))
    }

    pub async fn update<T, U>(
        &mut self,
        name: String,
        fields: T,
        current_update_time: Timestamp,
    ) -> Result<Document<U>, Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let response = self
            .client
            .update_document(Request::new(UpdateDocumentRequest {
                document: Some(FirestoreDocument {
                    name,
                    fields: {
                        let ser = to_value(&fields)?;
                        if let Some(ValueType::MapValue(MapValue { fields })) = ser.value_type {
                            fields
                        } else {
                            return Err(Error::ValueType);
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
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }
}

#[cfg(test)]
mod tests {
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
        let (documents, _) = client.list::<V>(collection_name.to_owned()).await?;
        for doc in documents {
            client.delete(doc.clone().name(), doc.update_time()).await?;
        }

        // CREATE
        #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct V {
            k1: String,
        }
        let input = V {
            k1: "v1".to_string(),
        };
        let created = client
            .create(collection_name.to_string(), input.clone())
            .await?;
        assert!(created
            .clone()
            .name()
            .starts_with("projects/demo-project1/databases/(default)/documents/repositories/"),);
        assert_eq!(created.clone().data(), input);

        // READ (GET)
        let got = client.get(created.clone().name()).await?;
        assert_eq!(got, created);

        // READ (LIST)
        let (documents, next_page_token) = client.list::<V>(collection_name.to_owned()).await?;
        assert_eq!(documents, vec![got.clone()]);
        assert_eq!(next_page_token, "");

        // UPDATE
        let updated: Document<V> = client
            .update(
                got.clone().name(),
                V {
                    k1: "v2".to_owned(), // "v1" -> "v2
                },
                got.update_time().clone(),
            )
            .await?;
        assert_eq!(
            updated.clone().data(),
            V {
                k1: "v2".to_string()
            }
        );

        // DELETE
        client
            .delete(updated.clone().name(), updated.clone().update_time())
            .await?;

        Ok(())
    }
}
