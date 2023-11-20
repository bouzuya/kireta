use async_graphql::Context;

use crate::{
    handler::graphql::graphql_data::GraphQLData,
    model::{self},
};

use super::check_list::CheckList;

#[derive(Clone, Debug)]
pub struct Item(pub model::Item);

#[async_graphql::Object]
impl Item {
    async fn id(&self) -> &str {
        &self.0.id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn checked_check_lists(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<CheckList>> {
        let store = &context.data_unchecked::<GraphQLData>().store;
        let check_lists = store.find_all_check_lists().await?;
        // TODO: Store::find_checks_by_item_id
        let checks = store.find_all_checks().await?;
        Ok(checks
            .into_iter()
            .filter(|check| check.item_id == self.0.id)
            .map(|check| {
                check_lists
                    .iter()
                    .find(|check_list| check_list.id == check.check_list_id)
                    .cloned()
                    .expect("check list not found")
            })
            .map(CheckList)
            .collect())
    }
}
