use async_graphql::{http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing, Router, Server,
};

#[derive(Clone, Debug)]
struct Item<'a> {
    id: &'a str,
    name: &'a str,
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

struct Query;

#[async_graphql::Object]
impl Query {
    async fn hello(&self) -> &'static str {
        "Hello, World!"
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn items<'a>(&self, ctx: &Context<'a>) -> Vec<Item<'a>> {
        let store = ctx.data_unchecked::<Store>();
        store.items.to_vec()
    }
}

struct Store {
    items: Vec<Item<'static>>,
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(Store {
            items: vec![
                Item {
                    id: "1",
                    name: "item1",
                },
                Item {
                    id: "2",
                    name: "item2",
                },
            ],
        })
        .finish();
    let app = Router::new()
        .route(
            "/graphql",
            routing::get(graphiql).post_service(GraphQL::new(schema)),
        )
        .route("/", routing::get(|| async { "Hello, World!" }));
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
