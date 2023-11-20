use super::{mutation::MutationRoot, query::QueryRoot};
use async_graphql::{EmptySubscription, Request, Response, Schema};

#[derive(Clone)]
pub struct GraphQLSchema(Schema<QueryRoot, MutationRoot, EmptySubscription>);

impl GraphQLSchema {
    pub fn new() -> Self {
        Self(Schema::new(QueryRoot, MutationRoot, EmptySubscription))
    }

    pub async fn execute<R>(&self, request: R) -> Response
    where
        R: Into<Request>,
    {
        self.0.execute(request).await
    }
}
