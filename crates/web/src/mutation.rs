use async_graphql::Context;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    pub async fn sign_in(
        &self,
        _context: &Context<'_>,
        user_id: String,
        password: String,
    ) -> String {
        // TODO: check user_id and password
        // TODO: jwt
        format!("{}:{}", user_id, password)
    }
}
