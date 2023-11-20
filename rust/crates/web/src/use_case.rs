use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use axum::async_trait;

use crate::{model, mutation};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[async_trait]
pub trait Store {
    async fn find_all_check_lists(&self) -> Result<Vec<model::CheckList>, Error>;
    async fn find_all_checks(&self) -> Result<Vec<model::Check>, Error>;
    async fn find_all_items(&self) -> Result<Vec<model::Item>, Error>;
    async fn find_checks_by_check_list_id(
        &self,
        check_list_id: String,
    ) -> Result<Vec<model::Check>, Error>;
}

pub trait HasSchema {
    // TODO: query, mutation, subscription
    fn schema(
        &self,
    ) -> &Schema<crate::handler::graphql::query::QueryRoot, mutation::MutationRoot, EmptySubscription>;
}

pub trait HasStore {
    fn store(&self) -> Arc<dyn Store + Send + Sync>;
}
