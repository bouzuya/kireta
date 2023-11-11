use async_graphql::{EmptySubscription, Schema};
use axum::async_trait;

use crate::{model, mutation, query};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[async_trait]
pub trait StoreTrait {
    async fn find_all_check_lists(&self) -> Result<Vec<model::CheckList>, Error>;
    async fn find_all_checks(&self) -> Result<Vec<model::Check>, Error>;
    async fn find_all_items(&self) -> Result<Vec<model::Item>, Error>;
}

pub trait HasSchema {
    // TODO: query, mutation, subscription
    fn schema(&self) -> &Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;
}

pub trait HasStore {
    type Store: StoreTrait + Send + Sync + 'static;
    fn store(&self) -> Self::Store;
}
