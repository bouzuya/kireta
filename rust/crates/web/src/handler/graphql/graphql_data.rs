use std::sync::Arc;

use axum::headers::authorization::Bearer;

use crate::use_case::Store;

pub struct GraphQLData {
    pub bearer: Option<Bearer>,
    pub store: Arc<dyn Store + Send + Sync>,
}
