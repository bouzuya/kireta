use async_graphql::{EmptySubscription, Schema};

use crate::{
    infra::store::InMemoryStore,
    mutation::MutationRoot,
    query::QueryRoot,
    use_case::{HasSchema, HasStore},
};

#[derive(Clone)]
pub struct App {
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    store: InMemoryStore,
}

impl App {
    pub fn example() -> Self {
        let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);
        Self {
            schema,
            store: InMemoryStore::example(),
        }
    }
}

impl HasSchema for App {
    fn schema(&self) -> &Schema<QueryRoot, MutationRoot, EmptySubscription> {
        &self.schema
    }
}

impl HasStore for App {
    type Store = InMemoryStore;
    fn store(&self) -> Self::Store {
        self.store.clone()
    }
}
