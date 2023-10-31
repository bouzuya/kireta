use async_graphql::Context;

use crate::{handler::Data, model};

use super::item::Item;

#[derive(Clone, Debug)]
pub struct CheckList(pub model::CheckList);

/// check list
#[async_graphql::Object]
impl CheckList {
    async fn id(&self) -> &str {
        &self.0.id
    }

    async fn date(&self) -> &str {
        &self.0.date
    }

    async fn checked_items(&self, context: &Context<'_>) -> Vec<Item> {
        let store = &context.data_unchecked::<Data>().0;
        let items = store.find_all_items().await;
        // TODO: Store::find_checks_by_check_list_id
        let checks = store.find_all_checks().await;
        checks
            .into_iter()
            .filter(|check| check.check_list_id == self.0.id)
            .map(|check| {
                items
                    .iter()
                    .find(|item| item.id == check.item_id)
                    .cloned()
                    .unwrap()
            })
            .map(Item)
            .collect()
    }
}
