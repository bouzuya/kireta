mod item;
mod store;

use async_graphql::Context;

use self::item::Item;

pub use self::store::Store;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> &'static str {
        "Hello, World!"
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn items<'a>(&self, ctx: &Context<'a>) -> Vec<Item<'a>> {
        let store = ctx.data_unchecked::<Store>();
        store.items.to_vec()
    }
}
