use std::{future::Future, pin::Pin};

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

use crate::infra::firestore::document;

use super::{
    document::Document,
    path::{self, CollectionPath, DocumentPath, RootPath},
    timestamp::Timestamp,
};

pub struct Transaction {
    client: Client,
    transaction: prost::bytes::Bytes,
    writes: Vec<Write>,
}

impl Transaction {
    pub async fn commit(mut self) -> Result<((), Option<Timestamp>), Error> {
        let response = self
            .client
            .client
            .commit(CommitRequest {
                database: self.client.root_path.database_name(),
                writes: self.writes,
                transaction: self.transaction,
            })
            .await?;
        // TODO: write_results
        let CommitResponse { commit_time, .. } = response.into_inner();
        Ok(((), commit_time.map(Timestamp::from)))
    }

    pub fn create<T>(&mut self, document_path: &DocumentPath, fields: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.writes.push(Write {
            operation: Some(Operation::Update(
                google_api_proto::google::firestore::v1::Document {
                    name: document_path.path(),
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
        document_path: &DocumentPath,
        current_update_time: Timestamp,
    ) -> Result<(), Error> {
        self.writes.push(Write {
            operation: Some(Operation::Delete(document_path.path())),
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

    pub async fn get<U>(&mut self, document_path: &DocumentPath) -> Result<Document<U>, Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .client
            .get_document(GetDocumentRequest {
                name: document_path.path(),
                mask: None,
                consistency_selector: Some(ConsistencySelector::Transaction(
                    self.transaction.clone(),
                )),
            })
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }

    pub async fn rollback(mut self) -> Result<(), Error> {
        self.client
            .client
            .rollback(RollbackRequest {
                database: self.client.root_path.database_name(),
                transaction: self.transaction,
            })
            .await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("credentials {0}")]
    Credentials(#[from] google_authz::CredentialsError),
    #[error("deserialize {0}")]
    Deserialize(#[from] document::Error),
    #[error("path {0}")]
    Path(#[from] path::Error),
    #[error("serialize {0}")]
    Serialize(#[from] serde_firestore_value::Error),
    #[error("status {0}")]
    Status(#[from] tonic::Status),
    #[error("transport {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("value_type")]
    ValueType,
}

#[derive(Clone)]
pub struct Client {
    client: FirestoreClient<GoogleAuthz<Channel>>,
    root_path: RootPath,
}

impl Client {
    // TODO: run_query

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
        let root_path = RootPath::new(project_id, database_id)?;
        Ok(Self { client, root_path })
    }

    pub async fn begin_transaction(&mut self) -> Result<Transaction, Error> {
        let response = self
            .client
            .begin_transaction(BeginTransactionRequest {
                database: self.root_path.database_name(),
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

    pub fn collection(&self, collection_id: String) -> CollectionPath {
        self.root_path.clone().collection(collection_id)
    }

    pub async fn create<T, U>(
        &mut self,
        document_path: &DocumentPath,
        fields: T,
    ) -> Result<Document<U>, Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let response = self
            .client
            .create_document(CreateDocumentRequest {
                parent: document_path.parent().parent().path(),
                collection_id: document_path.parent().id().to_string(),
                document_id: document_path.id().to_string(),
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
        document_path: &DocumentPath,
        current_update_time: Timestamp,
    ) -> Result<(), Error> {
        self.client
            .delete_document(DeleteDocumentRequest {
                name: document_path.path(),
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::UpdateTime(prost_types::Timestamp::from(
                        current_update_time,
                    ))),
                }),
            })
            .await?;
        Ok(())
    }

    pub async fn get<U>(&mut self, document_path: &DocumentPath) -> Result<Document<U>, Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .get_document(GetDocumentRequest {
                name: document_path.path(),
                mask: None,
                consistency_selector: None,
            })
            .await?;
        Document::new(response.into_inner()).map_err(Error::Deserialize)
    }

    // TODO: support some params
    pub async fn list<U>(
        &mut self,
        collection_path: &CollectionPath,
    ) -> Result<(Vec<Document<U>>, String), Error>
    where
        U: DeserializeOwned,
    {
        let response = self
            .client
            .list_documents(ListDocumentsRequest {
                parent: collection_path.parent().path(),
                collection_id: collection_path.id().to_string(),
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
            .map(|documents| (documents, next_page_token))
    }

    pub async fn run_transaction<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Transaction) -> Pin<Box<dyn Future<Output = Result<(), Error>> + '_>>,
    {
        let response = self
            .client
            .begin_transaction(BeginTransactionRequest {
                database: self.root_path.database_name(),
                options: None,
            })
            .await?;
        let BeginTransactionResponse { transaction } = response.into_inner();
        let mut transaction = Transaction {
            client: self.clone(),
            transaction,
            writes: vec![],
        };
        match f(&mut transaction).await {
            Ok(()) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }

    pub async fn update<T, U>(
        &mut self,
        document_path: &DocumentPath,
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
                    name: document_path.path(),
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
