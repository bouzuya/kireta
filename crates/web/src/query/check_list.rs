use crate::model;

#[derive(Clone, Debug)]
pub struct CheckList<'a>(pub &'a model::CheckList);

#[async_graphql::Object]
impl<'a> CheckList<'a> {
    async fn id(&self) -> &str {
        &self.0.id
    }

    async fn date(&self) -> &str {
        &self.0.date
    }
}
