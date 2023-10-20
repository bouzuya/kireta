#[derive(Clone, Debug)]
pub struct Item<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[async_graphql::Object]
impl<'a> Item<'a> {
    async fn id(&self) -> &str {
        self.id
    }

    async fn name(&self) -> &str {
        self.name
    }
}
