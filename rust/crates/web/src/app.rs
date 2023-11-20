use std::sync::Arc;

use crate::{
    handler::{graphql::GraphQLSchema, HasGraphQLSchema},
    infra::store::InMemoryStore,
    use_case::{HasStore, Store},
};

#[derive(Clone)]
pub struct App {
    graphql_schema: GraphQLSchema,
    store: Arc<InMemoryStore>,
}

impl App {
    pub fn example() -> Self {
        Self {
            graphql_schema: GraphQLSchema::new(),
            store: Arc::new(InMemoryStore::example()),
        }
    }
}

impl HasGraphQLSchema for App {
    fn graphql_schema(&self) -> &GraphQLSchema {
        &self.graphql_schema
    }
}

impl HasStore for App {
    fn store(&self) -> Arc<dyn Store + Send + Sync> {
        self.store.clone()
    }
}
