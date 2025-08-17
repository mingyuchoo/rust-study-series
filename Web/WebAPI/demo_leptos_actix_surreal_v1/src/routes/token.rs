use crate::routes::ServerError;
use actix_web::get;
use faker_rand::en_us::names::FirstName;
use memo_leptos_actix_surreal_v1::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Record;

#[derive(Serialize, Deserialize)]
struct Params<'a> {
    name: &'a str,
    pass: &'a str,
}

#[get("/new_user")]
pub async fn make_new_user() -> Result<String, ServerError> {
    let name = rand::random::<FirstName>().to_string();
    let pass = rand::random::<FirstName>().to_string();
    let jwt = DB
        .signup(Record {
            access:    "account",
            namespace: "namespace",
            database:  "database",
            params:    Params {
                name: &name,
                pass: &pass,
            },
        })
        .await?
        .into_insecure_token();
    Ok(format!(
        "New user created!\n\nName: {name}\nPassword: \
                {pass}\nToken: {jwt}\n\nTo log in, use this \
                command:\n\nsurreal sql --namespace namespace \
                --database database --pretty --token \"{jwt}\""
    ))
}

#[get("/new_token")]
pub async fn get_new_token() -> String {
    let command = r#"curl -X POST -H "Accept: application/json" -d '{"ns":"namespace","db":"database","ac":"account","user":"your_username","pass":"your_password"}' http://localhost:8000/signin"#;
    format!(
        "Need a new token? Use this command:\n\n{command}\n\nThen log in \
             with surreal sql --namespace namespace --database database \
             --pretty --token YOUR_TOKEN_HERE"
    )
}
