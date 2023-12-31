pub mod graphql;
mod root;

use axum::Router;

pub use self::graphql::HasGraphQLSchema;
use crate::use_case::HasStore;

pub fn route<T: Clone + HasGraphQLSchema + HasStore + Send + Sync + 'static>() -> Router<T> {
    Router::new()
        .merge(graphql::route::<T>())
        .merge(root::route::<T>())
}
