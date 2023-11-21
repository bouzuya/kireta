use std::sync::Arc;

use axum::async_trait;

use crate::{
    model::{self, CheckList},
    use_case::{self, Store},
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
impl Store for FirestoreStore {
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

    async fn find_checks_by_check_list_id(
        &self,
        check_list_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::firestore::document::Document;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let mut client = Client::new(
            "demo-project1".to_string(),
            "(default)".to_string(),
            endpoint,
        )
        .await?;
        let collection = client.collection("check_lists")?;
        let doc = collection.doc("1")?;

        let input = CheckListDocumentData {
            date: "2020-01-02".to_string(),
            id: "1".to_string(),
        };
        let created: Document<CheckListDocumentData> = client.create(&doc, input).await?;

        let store = FirestoreStore {
            client: Arc::new(tokio::sync::Mutex::new(client.clone())),
        };
        let found = store.find_all_check_lists().await?;
        assert_eq!(
            found,
            vec![CheckList {
                id: "1".to_string(),
                date: "2020-01-02".to_string()
            }]
        );

        client.delete(&doc, created.update_time()).await?;

        Ok(())
    }
}
