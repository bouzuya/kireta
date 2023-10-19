use crate::item::Item;

pub struct Store {
    pub items: Vec<Item<'static>>,
}

impl Store {
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
