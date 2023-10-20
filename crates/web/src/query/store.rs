use super::item::Item;

pub struct Store {
    items: Vec<Item<'static>>,
}

impl Store {
    pub async fn find_all_items(&self) -> Vec<Item<'_>> {
        self.items.to_vec()
    }

    pub fn example() -> Self {
        Self {
            items: vec![
                Item {
                    id: "1",
                    name: "item1",
                },
                Item {
                    id: "2",
                    name: "item2",
                },
            ],
        }
    }
}
