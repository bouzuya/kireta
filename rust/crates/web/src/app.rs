use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};

use crate::{
    infra::store::InMemoryStore,
    mutation::MutationRoot,
    query::QueryRoot,
    use_case::{HasSchema, HasStore, Store},
};

#[derive(Clone)]
pub struct App {
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    store: Arc<InMemoryStore>,
}

impl App {
    pub fn example() -> Self {
        let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);
        Self {
            schema,
            store: Arc::new(InMemoryStore::example()),
        }
    }
}

impl HasSchema for App {
    fn schema(&self) -> &Schema<QueryRoot, MutationRoot, EmptySubscription> {
        &self.schema
    }
}

impl HasStore for App {
    fn store(&self) -> Arc<dyn Store + Send + Sync> {
        self.store.clone()
    }
}
