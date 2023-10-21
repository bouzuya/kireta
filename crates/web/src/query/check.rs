use crate::model;

#[derive(Clone, Debug)]
pub struct Check<'a>(pub &'a model::Check);

#[async_graphql::Object]
impl<'a> Check<'a> {
    async fn check_list_id(&self) -> &str {
        &self.0.check_list_id
    }

    async fn item_id(&self) -> &str {
        &self.0.item_id
    }
}
