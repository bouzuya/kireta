mod check;
mod check_list;
mod item;

use async_graphql::Context;
use axum::headers::authorization::Bearer;

use crate::handler::Data;

use self::{check_list::CheckList, item::Item};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn bearer<'a>(&self, context: &Context<'a>) -> &'a str {
        let bearer = context.data_unchecked::<Bearer>();
        bearer.token()
    }

    async fn hello(&self) -> &'static str {
        "Hello, World!"
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn check_lists<'a>(&self, context: &Context<'a>) -> Vec<CheckList> {
        let store = &context.data_unchecked::<Data>().0;
        store
            .find_all_check_lists()
            .await
            .unwrap()
            .into_iter()
            .map(CheckList)
            .collect()
    }

    async fn items<'a>(&self, ctx: &Context<'a>) -> Vec<Item> {
        let store = &ctx.data_unchecked::<Data>().0;
        store
            .find_all_items()
            .await
            .unwrap()
            .into_iter()
            .map(Item)
            .collect()
    }
}
