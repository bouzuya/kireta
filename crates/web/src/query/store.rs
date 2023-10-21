use crate::model;

use super::{check_list::CheckList, item::Item};

pub struct Store {
    check_lists: Vec<model::CheckList>,
    items: Vec<model::Item>,
}

impl Store {
    pub async fn find_all_check_lists(&self) -> Vec<CheckList<'_>> {
        self.check_lists.iter().map(CheckList).collect()
    }

    pub async fn find_all_items(&self) -> Vec<Item<'_>> {
        self.items.iter().map(Item).collect()
    }

    pub fn example() -> Self {
        Self {
            check_lists: vec![model::CheckList {
                id: "1".to_owned(),
                date: "2020-01-02".to_owned(),
            }],
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
