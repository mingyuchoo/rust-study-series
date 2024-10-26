use diesel::Connection;

#[derive(diesel::prelude::Queryable)]
pub struct Member
{
    pub id:        i32,
    pub name:      String,
    pub knockouts: i32,
    pub team_id:   i32,
}

#[async_graphql::Object]
impl Member
{
    pub async fn id(&self) -> i32
    {
        self.id
    }

    pub async fn name(&self) -> &str
    {
        self.name
            .as_str()
    }

    pub async fn knockouts(&self) -> i32
    {
        self.knockouts
    }

    pub async fn team_id(&self) -> i32
    {
        self.team_id
    }
}

#[derive(diesel::prelude::Queryable)]
pub struct Team
{
    pub id:   i32,
    pub name: String,
}

#[async_graphql::Object]
impl Team
{
    pub async fn id(&self) -> i32
    {
        self.id
    }

    pub async fn name(&self) -> &str
    {
        self.name
            .as_str()
    }

    pub async fn members(&self) -> Vec<Member>
    {
        vec![]
    }
}

fn establish_connection() -> diesel::pg::PgConnection
{
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::pg::PgConnection::establish(&database_url).expect(&format!("Error connection to {}",
                                                                       database_url))
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot
{
    async fn members(&self) -> Vec<Member>
    {
        use diesel::{query_dsl::methods::LimitDsl,
                     RunQueryDsl};

        let connection = &mut establish_connection();
        crate::diesel_schema::members::dsl::members.limit(100)
                                                   .load::<Member>(connection)
                                                   .expect("Error loading members")
    }
}

pub type Schema = async_graphql::Schema<QueryRoot,
                                        async_graphql::EmptyMutation,
                                        async_graphql::EmptySubscription>;

pub fn create_schema() -> Schema
{
    async_graphql::Schema::new(QueryRoot,
                               async_graphql::EmptyMutation,
                               async_graphql::EmptySubscription)
}
