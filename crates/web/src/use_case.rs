use async_graphql::{EmptySubscription, Schema};
use axum::async_trait;

use crate::{model, mutation, query};

#[async_trait]
pub trait StoreTrait {
    async fn find_all_check_lists(&self) -> Vec<model::CheckList>;
    async fn find_all_checks(&self) -> Vec<model::Check>;
    async fn find_all_items(&self) -> Vec<model::Item>;
}

pub trait HasSchema {
    // TODO: query, mutation, subscription
    fn schema(&self) -> Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;
}
