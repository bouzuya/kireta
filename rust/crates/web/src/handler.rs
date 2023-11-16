mod graphql;
mod root;

use axum::Router;

pub use self::graphql::Data;
use crate::use_case::{HasSchema, HasStore};

pub fn route<T: Clone + HasSchema + HasStore + Send + Sync + 'static>(store: T) -> Router {
    Router::new()
        .merge(graphql::route::<T>())
        .merge(root::route::<T>())
        .with_state(store)
}
