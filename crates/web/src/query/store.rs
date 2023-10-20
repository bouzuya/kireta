use crate::model;

use super::item::Item;

pub struct Store {
    items: Vec<model::Item>,
}

impl Store {
    pub async fn find_all_items(&self) -> Vec<Item<'_>> {
        self.items.iter().map(Item).collect()
    }

    pub fn example() -> Self {
        Self {
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
