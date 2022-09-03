// use std::env;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenv::dotenv;
// use crate::diesel_schema::members;

fn establish_connection() -> diesel::pg::PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::pg::PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

pub struct QueryRoot;

#[derive(diesel::prelude::Queryable)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

#[async_graphql::Object]
impl Member {
    pub async fn id(&self) -> i32 {
        self.id
    }
    pub async fn name(&self) -> &str {
        self.name.as_str()
    }
    pub async fn knockouts(&self) -> i32 {
        self.knockouts
    }
    pub async fn team_id(&self) -> i32 {
        self.team_id
    }
}

#[derive(diesel::prelude::Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[async_graphql::Object]
impl Team {
    pub async fn id(&self) -> i32 {
        self.id
    }
    pub async fn name(&self) -> &str {
        self.name.as_str()
    }
    pub async fn members(&self) -> Vec<Member> {
        vec![]
    }
}

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn members(&self) -> Vec<Member> {
        vec![
            Member {
                id: 1,
                name: "Link".to_owned(),
                knockouts: 1,
                team_id: 1,
            },
            Member {
                id: 2,
                name: "Mario".to_owned(),
                knockouts: 2,
                team_id: 2,
            },
        ]
    }
}


pub type Schema = async_graphql::Schema<
    QueryRoot,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;

pub fn create_schema() -> Schema {
    async_graphql::Schema::new(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
}
