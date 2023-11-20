mod check;
mod check_list;
mod item;

use async_graphql::Context;

use crate::handler::Data;

use self::{check_list::CheckList, item::Item};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn bearer<'a>(&self, context: &Context<'a>) -> &'a str {
        let data = context.data_unchecked::<Data>();
        data.bearer.as_ref().unwrap().token()
    }

    async fn hello(&self) -> &'static str {
        "Hello, World!"
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn check_lists<'a>(
        &self,
        context: &Context<'a>,
    ) -> async_graphql::Result<Vec<CheckList>> {
        let store = &context.data_unchecked::<Data>().store;
        Ok(store
            .find_all_check_lists()
            .await?
            .into_iter()
            .map(CheckList)
            .collect())
    }

    async fn items<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Vec<Item>> {
        let store = &ctx.data_unchecked::<Data>().store;
        Ok(store
            .find_all_items()
            .await?
            .into_iter()
            .map(Item)
            .collect())
    }
}
