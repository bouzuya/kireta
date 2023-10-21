use crate::model;

pub struct Store {
    check_lists: Vec<model::CheckList>,
    checks: Vec<model::Check>,
    items: Vec<model::Item>,
}

impl Store {
    pub async fn find_all_check_lists(&self) -> Vec<model::CheckList> {
        self.check_lists.clone()
    }

    pub async fn find_all_checks(&self) -> Vec<model::Check> {
        self.checks.clone()
    }

    pub async fn find_all_items(&self) -> Vec<model::Item> {
        self.items.clone()
    }

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
