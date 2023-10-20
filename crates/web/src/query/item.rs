use crate::model;

#[derive(Clone, Debug)]
pub struct Item<'a>(pub &'a model::Item);

#[async_graphql::Object]
impl<'a> Item<'a> {
    async fn id(&self) -> &str {
        &self.0.id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }
}
