use axum::async_trait;

use crate::{
    model,
    use_case::{self, Store},
};

#[derive(Clone, Debug)]
pub struct InMemoryStore {
    check_lists: Vec<model::CheckList>,
    checks: Vec<model::Check>,
    items: Vec<model::Item>,
}

#[async_trait]
impl Store for InMemoryStore {
    async fn find_all_check_lists(&self) -> Result<Vec<model::CheckList>, use_case::Error> {
        Ok(self.check_lists.clone())
    }

    async fn find_all_checks(&self) -> Result<Vec<model::Check>, use_case::Error> {
        Ok(self.checks.clone())
    }

    async fn find_all_items(&self) -> Result<Vec<model::Item>, use_case::Error> {
        Ok(self.items.clone())
    }

    async fn find_checks_by_check_list_id(
        &self,
        check_list_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        Ok(self
            .checks
            .iter()
            .filter(|check| check.check_list_id == check_list_id)
            .cloned()
            .collect())
    }

    async fn find_checks_by_item_id(
        &self,
        item_id: String,
    ) -> Result<Vec<model::Check>, use_case::Error> {
        Ok(self
            .checks
            .iter()
            .filter(|check| check.item_id == item_id)
            .cloned()
            .collect())
    }
}

impl InMemoryStore {
    pub fn example() -> Self {
        Self {
            check_lists: vec![
                model::CheckList {
                    id: "1".to_owned(),
                    date: "2020-01-02".to_owned(),
                },
                model::CheckList {
                    id: "2".to_owned(),
                    date: "2020-01-03".to_owned(),
                },
            ],
            checks: vec![
                model::Check {
                    check_list_id: "1".to_owned(),
                    item_id: "1".to_owned(),
                },
                model::Check {
                    check_list_id: "2".to_owned(),
                    item_id: "2".to_owned(),
                },
            ],
            items: vec![
                model::Item {
                    id: "1".to_owned(),
                    name: "item1".to_owned(),
                },
                model::Item {
                    id: "2".to_owned(),
                    name: "item2".to_owned(),
                },
            ],
        }
    }
}
