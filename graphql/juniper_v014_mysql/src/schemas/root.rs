use juniper::{FieldError,
              FieldResult,
              RootNode};
use mysql::{from_row,
            params,
            prelude::*,
            Error as DBError,
            Row};

use crate::db::DBPool;

use super::{product::{Product,
                      ProductInput},
            user::{User,
                   UserInput}};

pub struct Context
{
    pub dbpool: DBPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot
{
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<User>>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let users = conn.query_map("select id, name, email from user", |(id, name, email)| {
                            User { id:    id,
                                   name:  name,
                                   email: email, }
                        })
                        .unwrap();

        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context,
            id: String)
            -> FieldResult<User>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let user: Result<Option<Row>, DBError> =
            conn.exec_first("SELECT * FROM user WHERE id=:id", params! {"id" => id});

        if let Err(err) = user {
            return Err(FieldError::new("User Not Found",
                                       graphql_value!({ "not_found": "user not found" })));
        }

        let (id, name, email) = from_row(user.unwrap()
                                             .unwrap());
        Ok(User { id, name, email })
    }

    #[graphql(description = "List of all products")]
    fn products(context: &Context) -> FieldResult<Vec<Product>>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let products = conn.query_map("SELECT * FROM product", |(id, user_id, name, price)| {
                               Product { id,
                                         user_id,
                                         name,
                                         price }
                           })
                           .unwrap();
        Ok(products)
    }

    #[graphql(description = "Get Single product reference by product ID")]
    fn product(context: &Context,
               id: String)
               -> FieldResult<Product>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let product: Result<Option<Row>, DBError> =
            conn.exec_first("SELECT * FROM product WHERE id=:id", params! {"id" => id});
        if let Err(err) = product {
            return Err(FieldError::new("Product Not Found",
                                       graphql_value!({ "not_found": "product not found" })));
        }

        let (id, user_id, name, price) = from_row(product.unwrap()
                                                         .unwrap());
        Ok(Product { id,
                     user_id,
                     name,
                     price })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot
{
    #[graphql(description = "Create Single user")]
    fn create_user(context: &Context,
                   user: UserInput)
                   -> FieldResult<User>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple()
                                         .to_string();

        let insert: Result<Option<Row>, DBError> =
            conn.exec_first("INSERT INTO user(id, name, email) VALUES(:id, :name, :email)",
                            params! {
                                "id" => &new_id,
                                "name" => &user.name,
                                "email" => &user.email,
                            });

        match insert {
            | Ok(opt_row) => Ok(User { id:    new_id,
                                       name:  user.name,
                                       email: user.email, }),
            | Err(err) => {
                let msg = match err {
                    | DBError::MySqlError(err) => err.message,
                    | _ => "internal error".to_owned(),
                };
                Err(FieldError::new("Failed to create new user",
                                    graphql_value!({ "internal_error": msg })))
            },
        }
    }

    // TODO: handling when deleting data not exists
    #[graphql(description = "Delete Single user")]
    fn delete_user(context: &Context,
                   user: UserInput)
                   -> FieldResult<User>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();

        let delete: Result<Option<Row>, DBError> =
            conn.exec_first("DELETE FROM user WHERE name = :name AND email = :email",
                            params! {
                                "name" => &user.name,
                                "email" => &user.email,
                            });
        match delete {
            | Ok(opt_row) => Ok(User { id:    "".to_string(),
                                       name:  user.name,
                                       email: user.email, }),
            | Err(err) => {
                let msg = match err {
                    | DBError::MySqlError(err) => err.message,
                    | _ => "internal error".to_owned(),
                };
                Err(FieldError::new("Failed to delete a user",
                                    graphql_value!({ "internal_error": msg })))
            },
        }
    }

    #[graphql(description = "Create Single product")]
    fn create_product(context: &Context,
                      product: ProductInput)
                      -> FieldResult<Product>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple()
                                         .to_string();

        let insert: Result<Option<Row>, DBError> = conn.exec_first(
            "INSERT INTO product(id, user_id, name, price) VALUES(:id, :user_id, :name, :price)",
            params! {
                "id" => &new_id,
                "user_id" => &product.user_id,
                "name" => &product.name,
                "price" => &product.price.to_owned(),
            },
        );

        match insert {
            | Ok(opt_row) => Ok(Product { id:      new_id,
                                          user_id: product.user_id,
                                          name:    product.name,
                                          price:   product.price, }),
            | Err(err) => {
                let msg = match err {
                    | DBError::MySqlError(err) => err.message,
                    | _ => "internal error".to_owned(),
                };
                Err(FieldError::new("Failed to create new product",
                                    graphql_value!({ "internal_error": msg })))
            },
        }
    }

    // TODO: handling when deleting data not exists
    #[graphql(description = "Delete Single product")]
    fn delete_product(context: &Context,
                      product: ProductInput)
                      -> FieldResult<Product>
    {
        let mut conn = context.dbpool
                              .get_conn()
                              .unwrap();

        let insert: Result<Option<Row>, DBError> = conn.exec_first(
            "DELETE FROM product WHERE user_id = :user_id AND name = :name AND price = :price",
            params! {
                "user_id" => &product.user_id,
                "name" => &product.name,
                "price" => &product.price.to_owned(),
            },
        );

        match insert {
            | Ok(opt_row) => Ok(Product { id:      "".to_string(),
                                          user_id: product.user_id,
                                          name:    product.name,
                                          price:   product.price, }),
            | Err(err) => {
                let msg = match err {
                    | DBError::MySqlError(err) => err.message,
                    | _ => "internal error".to_owned(),
                };
                Err(FieldError::new("Failed to delete a product",
                                    graphql_value!({ "internal_error": msg })))
            },
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema
{
    Schema::new(QueryRoot, MutationRoot)
}
