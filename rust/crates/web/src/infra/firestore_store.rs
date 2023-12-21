use std::sync::Arc;

use axum::async_trait;

use crate::{
    model::{self, Check, CheckList, Item},
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CheckDocumentData {
    pub check_list_id: String,
    pub item_id: String,
}

impl From<CheckDocumentData> for model::Check {
    fn from(
        CheckDocumentData {
            check_list_id,
            item_id,
        }: CheckDocumentData,
    ) -> Self {
        Self {
            check_list_id,
            item_id,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ItemDocumentData {
    pub id: String,
    pub name: String,
}

impl From<ItemDocumentData> for model::Item {
    fn from(ItemDocumentData { id, name }: ItemDocumentData) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("client {0}")]
    Client(#[from] super::firestore::client::Error),
    #[error("invalid path {0}")]
    InvalidPath(#[from] firestore_path::Error),
}

impl From<Error> for use_case::Error {
    fn from(e: Error) -> Self {
        Self::Unknown(e.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct FirestoreStore {
    client: Arc<tokio::sync::Mutex<Client>>,
}

impl FirestoreStore {
    async fn find_all_check_lists(&self) -> Result<Vec<model::CheckList>, Error> {
        let mut client = self.client.lock().await;
        let collection_name = client.collection("check_lists")?;
        // TODO: pagination
        Ok(client
            .list::<CheckListDocumentData>(&collection_name)
            .await?
            .0
            .into_iter()
            .map(|doc| doc.data())
            .map(CheckList::from)
            .collect())
    }

    async fn find_all_checks(&self) -> Result<Vec<model::Check>, Error> {
        let mut client = self.client.lock().await;
        let collection_name = client.collection("checks")?;
        Ok(client
            .list::<CheckDocumentData>(&collection_name)
            .await?
            .0
            .into_iter()
            .map(|doc| doc.data())
            .map(Check::from)
            .collect())
    }

    async fn find_all_items(&self) -> Result<Vec<model::Item>, Error> {
        let mut client = self.client.lock().await;
        let collection_name = client.collection("items")?;
        // TODO: pagination
        Ok(client
            .list::<ItemDocumentData>(&collection_name)
            .await?
            .0
            .into_iter()
            .map(|doc| doc.data())
            .map(Item::from)
            .collect())
    }

    async fn find_checks_by_check_list_id(
        &self,
        check_list_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        // TODO: improve perfomance
        Ok(self
            .find_all_checks()
            .await?
            .into_iter()
            .filter(|check| check.check_list_id == check_list_id)
            .collect())
    }

    async fn find_checks_by_item_id(
        &self,
        item_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        // TODO: improve perfomance
        Ok(self
            .find_all_checks()
            .await?
            .into_iter()
            .filter(|check| check.item_id == item_id)
            .collect())
    }
}

#[async_trait]
impl Store for FirestoreStore {
    async fn find_all_check_lists(&self) -> Result<Vec<model::CheckList>, use_case::Error> {
        Ok(self.find_all_check_lists().await?)
    }

    // TODO: remove
    async fn find_all_checks(&self) -> Result<Vec<model::Check>, use_case::Error> {
        Ok(self.find_all_checks().await?)
    }

    async fn find_all_items(&self) -> Result<Vec<model::Item>, use_case::Error> {
        Ok(self.find_all_items().await?)
    }

    async fn find_checks_by_check_list_id(
        &self,
        check_list_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        Ok(self.find_checks_by_check_list_id(check_list_id).await?)
    }

    async fn find_checks_by_item_id(
        &self,
        item_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        Ok(self.find_checks_by_item_id(item_id).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use firestore_path::{DatabaseId, DatabaseName, ProjectId};

    use crate::infra::firestore::document::Document;

    use super::*;

    #[tokio::test]
    async fn test_find_all_check_lists() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let mut client = Client::new(
            DatabaseName::new(
                ProjectId::from_str("demo-project1")?,
                DatabaseId::from_str("(default)")?,
            ),
            endpoint,
        )
        .await?;
        let collection = client.collection("check_lists")?;
        let doc = collection.clone().doc("1")?;

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

    #[tokio::test]
    async fn test_find_all_items() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let mut client = Client::new(
            DatabaseName::new(
                ProjectId::from_str("demo-project1")?,
                DatabaseId::from_str("(default)")?,
            ),
            endpoint,
        )
        .await?;
        let collection = client.collection("items")?;
        let doc = collection.clone().doc("1")?;

        let input = ItemDocumentData {
            id: "1".to_string(),
            name: "name1".to_string(),
        };
        let created: Document<ItemDocumentData> = client.create(&doc, input).await?;

        let store = FirestoreStore {
            client: Arc::new(tokio::sync::Mutex::new(client.clone())),
        };
        let found = store.find_all_items().await?;
        assert_eq!(
            found,
            vec![Item {
                id: "1".to_string(),
                name: "name1".to_string()
            }]
        );

        client.delete(&doc, created.update_time()).await?;

        Ok(())
    }
}
