use crate::schemas::root::Context;
use crate::schemas::user::User;
use mysql::prelude::*;
use mysql::{from_row, params};

/// Product
#[derive(Default, Debug)]
pub struct Product {
    pub id:      String,
    pub user_id: String,
    pub name:    String,
    pub price:   f64,
}

#[juniper::object(Context = Context)]
impl Product {
    fn id(&self) -> &str {
        &self.id
    }

    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn user(&self,
            context: &Context)
            -> Option<User> {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let user = conn.exec_first("SELECT * FROM user WHERE id=:id",
                                   params! {"id" => &self.user_id});
        if let Err(err) = user {
            None
        }
        else {
            let (id, name, email) = from_row(user.unwrap()
                                                 .unwrap());
            Some(User { id,
                        name,
                        email })
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub user_id: String,
    pub name:    String,
    pub price:   f64,
}
