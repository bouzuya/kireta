use std::sync::Arc;

use axum::async_trait;

use crate::{
    model::{self, CheckList},
    use_case::{self, StoreTrait},
};

use super::firestore::client::Client;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CheckListDocumentData {
    pub date: String,
    pub id: String,
}

impl From<CheckListDocumentData> for model::CheckList {
    fn from(CheckListDocumentData { date, id }: CheckListDocumentData) -> Self {
        Self { date, id }
    }
}

#[derive(Clone, Debug)]
pub struct FirestoreStore {
    client: Arc<tokio::sync::Mutex<Client>>,
}

#[async_trait]
impl StoreTrait for FirestoreStore {
    async fn find_all_check_lists(&self) -> Result<Vec<model::CheckList>, use_case::Error> {
        let mut client = self.client.lock().await;
        let collection_path = client.collection("check_lists")?;
        // TODO: pagination
        Ok(client
            .list::<CheckListDocumentData>(&collection_path)
            .await?
            .0
            .into_iter()
            .map(|doc| doc.data())
            .map(CheckList::from)
            .collect())
    }

    async fn find_all_checks(&self) -> Result<Vec<model::Check>, use_case::Error> {
        todo!()
    }

    async fn find_all_items(&self) -> Result<Vec<model::Item>, use_case::Error> {
        todo!()
    }
}
