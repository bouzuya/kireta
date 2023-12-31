use std::{future::Future, pin::Pin};

use firestore_path::{CollectionName, CollectionPath, DatabaseName, DocumentName};
use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, get_document_request::ConsistencySelector,
    precondition::ConditionType, value::ValueType, write::Operation, BeginTransactionRequest,
    BeginTransactionResponse, CommitRequest, CommitResponse, CreateDocumentRequest,
    DeleteDocumentRequest, GetDocumentRequest, ListDocumentsRequest, ListDocumentsResponse,
    MapValue, Precondition, RollbackRequest, UpdateDocumentRequest, Write,
};
use google_authz::{Credentials, GoogleAuthz};
use serde::{de::DeserializeOwned, Serialize};
use serde_firestore_value::to_value;
use tonic::transport::Channel;

use crate::{infra::firestore::document, use_case};

use super::{document::Document, timestamp::Timestamp};

pub struct Transaction {
    client: Client,
    transaction: prost::bytes::Bytes,
    writes: Vec<Write>,
}

impl Transaction {
    pub fn create<T>(&mut self, document_name: &DocumentName, fields: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.writes.push(Write {
            operation: Some(Operation::Update(
                google_api_proto::google::firestore::v1::Document {
                    name: document_name.to_string(),
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
                },
            )),
            update_mask: None,
            update_transforms: vec![],
            current_document: Some(Precondition {
                condition_type: Some(ConditionType::Exists(false)),
            }),
        });
        Ok(())
    }

    pub fn delete(
        &mut self,
        document_name: &DocumentName,
        current_update_time: Timestamp,
    ) -> Result<(), Error> {
        self.writes.push(Write {
            operation: Some(Operation::Delete(document_name.to_string())),
            update_mask: None,
            update_transforms: vec![],
            current_document: Some(Precondition {
                condition_type: Some(ConditionType::UpdateTime(prost_types::Timestamp::from(
                    current_update_time,
                ))),
            }),
        });
        Ok(())
    }

    pub async fn get<U>(&mut self, document_name: &DocumentName) -> Result<Document<U>, Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .client
            .get_document(GetDocumentRequest {
                name: document_name.to_string(),
                mask: None,
                consistency_selector: Some(ConsistencySelector::Transaction(
                    self.transaction.clone(),
                )),
            })
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("callback {0}")]
    Callback(Box<dyn std::error::Error + Send + Sync>),
    #[error("rollback {0} {1}")]
    Rollback(tonic::Status, Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("credentials {0}")]
    Credentials(#[from] google_authz::CredentialsError),
    #[error("deserialize {0}")]
    Deserialize(#[from] document::Error),
    #[error("path {0}")]
    Path(#[from] firestore_path::Error),
    #[error("serialize {0}")]
    Serialize(#[from] serde_firestore_value::Error),
    #[error("status {0}")]
    Status(#[from] tonic::Status),
    #[error("transaction {0}")]
    Transaction(#[from] TransactionError),
    #[error("transport {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("value_type")]
    ValueType,
}

impl From<Error> for use_case::Error {
    fn from(value: Error) -> Self {
        use_case::Error::Unknown(value.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    client: FirestoreClient<GoogleAuthz<Channel>>,
    database_name: DatabaseName,
}

impl Client {
    // TODO: run_query

    pub async fn new(database_name: DatabaseName, endpoint: &'static str) -> Result<Self, Error> {
        let credentials = Credentials::builder().no_credentials().build().await?;
        let channel = Channel::from_static(endpoint).connect().await?;
        let channel = GoogleAuthz::builder(channel)
            .credentials(credentials)
            .build()
            .await;
        let client = FirestoreClient::new(channel);
        Ok(Self {
            client,
            database_name,
        })
    }

    pub async fn begin_transaction(&mut self) -> Result<Transaction, Error> {
        let response = self
            .client
            .begin_transaction(BeginTransactionRequest {
                database: self.database_name.to_string(),
                options: None,
            })
            .await?;
        let BeginTransactionResponse { transaction } = response.into_inner();
        Ok(Transaction {
            client: self.clone(),
            transaction,
            writes: vec![],
        })
    }

    pub fn collection<S>(&self, collection_path: S) -> Result<CollectionName, Error>
    where
        S: TryInto<CollectionPath, Error = firestore_path::Error>,
    {
        Ok(self.database_name.clone().collection(collection_path)?)
    }

    pub async fn create<T, U>(
        &mut self,
        document_name: &DocumentName,
        fields: T,
    ) -> Result<Document<U>, Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let response = self
            .client
            .create_document(CreateDocumentRequest {
                parent: document_name
                    .clone()
                    .parent()
                    .parent()
                    .map(|parent| parent.to_string())
                    .unwrap_or_else(|| document_name.database_name().to_string()),
                collection_id: document_name.collection_id().to_string(),
                document_id: document_name.document_id().to_string(),
                document: Some(google_api_proto::google::firestore::v1::Document {
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
            })
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }

    pub async fn delete(
        &mut self,
        document_name: &DocumentName,
        current_update_time: Timestamp,
    ) -> Result<(), Error> {
        self.client
            .delete_document(DeleteDocumentRequest {
                name: document_name.to_string(),
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::UpdateTime(prost_types::Timestamp::from(
                        current_update_time,
                    ))),
                }),
            })
            .await?;
        Ok(())
    }

    pub async fn get<U>(&mut self, document_name: &DocumentName) -> Result<Document<U>, Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .get_document(GetDocumentRequest {
                name: document_name.to_string(),
                mask: None,
                consistency_selector: None,
            })
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }

    // TODO: support some params
    pub async fn list<U>(
        &mut self,
        collection_name: &CollectionName,
    ) -> Result<(Vec<Document<U>>, Option<String>), Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .list_documents(ListDocumentsRequest {
                parent: collection_name
                    .clone()
                    .parent()
                    .map(|parent| parent.to_string())
                    .unwrap_or_else(|| collection_name.database_name().to_string()),
                collection_id: collection_name.collection_id().to_string(),
                page_size: 100,
                ..Default::default()
            })
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
            .map(|documents| {
                (
                    documents,
                    (!next_page_token.is_empty()).then_some(next_page_token),
                )
            })
    }

    pub async fn run_transaction<F>(&mut self, callback: F) -> Result<(), Error>
    where
        F: FnOnce(
            &mut Transaction,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>
                    + Send
                    + '_,
            >,
        >,
    {
        let response = self
            .client
            .begin_transaction(BeginTransactionRequest {
                database: self.database_name.to_string(),
                options: None,
            })
            .await?;
        let BeginTransactionResponse { transaction } = response.into_inner();
        let mut transaction = Transaction {
            client: self.clone(),
            transaction,
            writes: vec![],
        };
        match callback(&mut transaction).await {
            Ok(()) => {
                let response = self
                    .client
                    .commit(CommitRequest {
                        database: self.database_name.to_string(),
                        writes: transaction.writes,
                        transaction: transaction.transaction,
                    })
                    .await?;
                // TODO: commit_time and write_results
                let CommitResponse { .. } = response.into_inner();
                Ok(())
            }
            Err(callback_err) => {
                match self
                    .client
                    .rollback(RollbackRequest {
                        database: self.database_name.to_string(),
                        transaction: transaction.transaction,
                    })
                    .await
                {
                    Ok(_) => Err(Error::Transaction(TransactionError::Callback(callback_err))),
                    Err(rollback_err) => Err(Error::Transaction(TransactionError::Rollback(
                        rollback_err,
                        callback_err,
                    ))),
                }
            }
        }
    }

    pub async fn update<T, U>(
        &mut self,
        document_name: &DocumentName,
        fields: T,
        current_update_time: Timestamp,
    ) -> Result<Document<U>, Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let response = self
            .client
            .update_document(UpdateDocumentRequest {
                document: Some(google_api_proto::google::firestore::v1::Document {
                    name: document_name.to_string(),
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
                    condition_type: Some(ConditionType::UpdateTime(prost_types::Timestamp::from(
                        current_update_time,
                    ))),
                }),
            })
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>()
    }
}
