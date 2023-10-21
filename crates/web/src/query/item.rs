use crate::model;

#[derive(Clone, Debug)]
pub struct Item(pub model::Item);

#[async_graphql::Object]
impl Item {
    async fn id(&self) -> &str {
        &self.0.id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }
}
