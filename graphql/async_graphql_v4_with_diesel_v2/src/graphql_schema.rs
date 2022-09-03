pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> Schema {
    async_graphql::Schema::new(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
}
