use actix_web::{App, HttpServer};
use std::sync::LazyLock;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

mod error {
    use actix_web::{HttpResponse, ResponseError};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db(String),
    }

    impl ResponseError for Error {
        fn error_response(&self) -> HttpResponse {
            match self {
                Error::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error}");
            Self::Db(error.to_string())
        }
    }
}

mod routes {

    use crate::{error::Error, DB};
    use actix_web::{
        delete, get, post, put,
        web::{Json, Path},
    };
    use faker_rand::en_us::names::FirstName;
    use serde::{Deserialize, Serialize};
    use surrealdb::{opt::auth::Record, RecordId};
    const PERSON: &str = "person";

    #[derive(Serialize, Deserialize)]
    pub struct PersonData {
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Person {
        name: String,
        id: RecordId,
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
        let res: Option<String> = DB.query("RETURN <string>$session").await?.take(0)?;

        Ok(Json(res.unwrap_or("No session data found!".into())))
    }

    #[post("/person/{id}")]
    pub async fn create_person(id: Path<String>, person: Json<PersonData>) -> Result<Json<Option<Person>>, Error> {
        let person = DB.create((PERSON, &*id)).content(person).await?;
        Ok(Json(person))
    }

    #[get("/person/{id}")]
    pub async fn read_person(id: Path<String>) -> Result<Json<Option<Person>>, Error> {
        let person = DB.select((PERSON, &*id)).await?;
        Ok(Json(person))
    }

    #[put("/person/{id}")]
    pub async fn update_person(id: Path<String>, person: Json<PersonData>) -> Result<Json<Option<Person>>, Error> {
        let person = DB.update((PERSON, &*id)).content(person).await?;
        Ok(Json(person))
    }

    #[delete("/person/{id}")]
    pub async fn delete_person(id: Path<String>) -> Result<Json<Option<Person>>, Error> {
        let person = DB.delete((PERSON, &*id)).await?;
        Ok(Json(person))
    }

    #[get("/people")]
    pub async fn list_people() -> Result<Json<Vec<Person>>, Error> {
        let people = DB.select(PERSON).await?;
        Ok(Json(people))
    }

    #[derive(Serialize, Deserialize)]
    struct Params<'a> {
        name: &'a str,
        pass: &'a str,
    }

    #[get("/new_user")]
    pub async fn make_new_user() -> Result<String, Error> {
        let name = rand::random::<FirstName>().to_string();
        let pass = rand::random::<FirstName>().to_string();
        let jwt = DB
            .signup(Record {
                access: "account",
                namespace: "namespace",
                database: "database",
                params: Params { name: &name, pass: &pass },
            })
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

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root { username: "root", password: "root" }).await?;

    DB.use_ns("namespace").use_db("database").await?;

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

    HttpServer::new(|| {
        App::new()
            .service(routes::create_person)
            .service(routes::read_person)
            .service(routes::update_person)
            .service(routes::delete_person)
            .service(routes::list_people)
            .service(routes::paths)
            .service(routes::session)
            .service(routes::make_new_user)
            .service(routes::get_new_token)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}
