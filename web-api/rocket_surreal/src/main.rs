#[macro_use]
extern crate rocket;

use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

mod error {
    use rocket::http::Status;
    use rocket::response::{self, Responder, Response};
    use rocket::Request;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db,
    }

    impl<'r> Responder<'r, 'static> for Error {
        fn respond_to(self,
                      _: &'r Request<'_>)
                      -> response::Result<'static> {
            let error_message = format!(r#"{{ "error": "{self}" }}"#);
            Response::build().status(Status::InternalServerError)
                             .header(rocket::http::ContentType::JSON)
                             .sized_body(error_message.len(),
                                         std::io::Cursor::new(error_message))
                             .ok()
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error}");
            Self::Db
        }
    }
}

mod routes {

    use crate::error::Error;
    use crate::DB;
    use faker_rand::en_us::names::FirstName;
    use rocket::serde::json::Json;
    use rocket::{delete, get, post, put};
    use serde::{Deserialize, Serialize};
    use surrealdb::opt::auth::Record;
    use surrealdb::RecordId;
    const PERSON: &str = "person";

    #[derive(Serialize, Deserialize)]
    struct Params<'a> {
        name: &'a str,
        pass: &'a str,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PersonData {
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Person {
        name: String,
        id:   RecordId,
    }

    #[get("/")]
    pub async fn paths() -> &'static str {
        r#"

-----------------------------------------------------------------------------------------------------------------------------------------
        PATH                |           SAMPLE COMMAND                                                                                  
-----------------------------------------------------------------------------------------------------------------------------------------
/session: See session data  |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/session
                            |
/person/{id}:               |
  Create a person           |  curl -X POST   -H "Content-Type: application/json" -d '{"name":"John Doe"}' http://localhost:8080/person/one
  Get a person              |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/person/one
  Update a person           |  curl -X PUT    -H "Content-Type: application/json" -d '{"name":"Jane Doe"}' http://localhost:8080/person/one
  Delete a person           |  curl -X DELETE -H "Content-Type: application/json"                          http://localhost:8080/person/one
                            |
/people: List all people    |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/people

/new_user:  Create a new record user
/new_token: Get instructions for a new token if yours has expired"#
    }

    #[get("/session")]
    pub async fn session() -> Result<Json<String>, Error> {
        let res: Option<String> = DB.query("RETURN <string>$session")
                                    .await?
                                    .take(0)?;

        Ok(Json(res.unwrap_or("No session data found!".into())))
    }

    #[post("/person/<id>", data = "<person>")]
    pub async fn create_person(id: String,
                               person: Json<PersonData>)
                               -> Result<Json<Option<Person>>, Error> {
        let person = DB.create((PERSON, &*id))
                       .content(person.into_inner())
                       .await?;
        Ok(Json(person))
    }

    #[get("/person/<id>")]
    pub async fn read_person(id: String)
                             -> Result<Json<Option<Person>>, Error> {
        let person = DB.select((PERSON, &*id))
                       .await?;
        Ok(Json(person))
    }

    #[put("/person/<id>", data = "<person>")]
    pub async fn update_person(id: String,
                               person: Json<PersonData>)
                               -> Result<Json<Option<Person>>, Error> {
        let person = DB.update((PERSON, &*id))
                       .content(person.into_inner())
                       .await?;
        Ok(Json(person))
    }

    #[delete("/person/<id>")]
    pub async fn delete_person(id: String)
                               -> Result<Json<Option<Person>>, Error> {
        let person = DB.delete((PERSON, &*id))
                       .await?;
        Ok(Json(person))
    }

    #[get("/people")]
    pub async fn list_people() -> Result<Json<Vec<Person>>, Error> {
        let people = DB.select(PERSON)
                       .await?;
        Ok(Json(people))
    }

    #[get("/new_user")]
    pub async fn make_new_user() -> Result<String, Error> {
        let name = rand::random::<FirstName>().to_string();
        let pass = rand::random::<FirstName>().to_string();
        let jwt = DB.signup(Record { access:    "account",
                                     namespace: "namespace",
                                     database:  "database",
                                     params:    Params { name: &name,
                                                         pass: &pass, }, })
                    .await?
                    .into_insecure_token();
        Ok(format!(
            "New user created!\n\nName: {name}\nPassword: {pass}\nToken: {jwt}\n\nTo log in, use this command:\n\nsurreal sql --namespace namespace --database database --pretty --token \"{jwt}\""
        ))
    }

    #[get("/new_token")]
    pub async fn get_new_token() -> String {
        let command = r#"curl -X POST -H "Accept: application/json" -d '{"ns":"namespace","db":"database","ac":"account","user":"your_username","pass":"your_password"}' http://localhost:8000/signin"#;
        format!("Need a new token? Use this command:\n\n{command}\n\nThen log in with surreal sql --namespace namespace --database database --pretty --token YOUR_TOKEN_HERE")
    }
}

async fn init() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8000")
      .await?;

    DB.signin(Root { username: "root",
                     password: "root", })
      .await?;

    DB.use_ns("namespace")
      .use_db("database")
      .await?;

    DB.query(
             "DEFINE TABLE person SCHEMALESS
            PERMISSIONS FOR 
                CREATE, SELECT WHERE $auth,
                FOR UPDATE, DELETE WHERE created_by = $auth;
        DEFINE FIELD name ON TABLE person TYPE string;
        DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;
        DEFINE INDEX unique_name ON TABLE user FIELDS name UNIQUE;
        DEFINE ACCESS account ON DATABASE TYPE RECORD
        SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
        SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
        DURATION FOR TOKEN 15m, FOR SESSION 12h
        ;",
    )
      .await?;
    Ok(())
}

#[launch]
pub async fn rocket() -> _ {
    std::env::set_var("ROCKET_PORT", "8080");
    init().await
          .expect("Something went wrong, shutting down");
    rocket::build().mount("/",
                          routes![routes::create_person,
                                  routes::read_person,
                                  routes::update_person,
                                  routes::delete_person,
                                  routes::list_people,
                                  routes::paths,
                                  routes::make_new_user,
                                  routes::get_new_token,
                                  routes::session])
}
