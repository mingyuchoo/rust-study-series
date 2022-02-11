use mysql::params;
use mysql::prelude::*;

use crate::schemas::product::Product;
use crate::schemas::root::Context;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn products(&self, context: &Context) -> Vec<Product> {
        let mut conn = context.dbpool.get_conn().unwrap();
        conn.exec_map(
            "SELECT * FROM product WHERE user_id=:user_id",
            params! { "user_id" => &self.id},
            |(id, user_id, name, price)| Product {
                id,
                user_id,
                name,
                price,
            },
        )
        .unwrap()
    }
}
