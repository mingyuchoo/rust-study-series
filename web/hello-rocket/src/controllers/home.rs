use rocket::get; 
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: String,
    pub first_name: String,
    pub last_name: String
}

#[get("/")]
pub fn index() -> Index {
  Index {
    title: "Index".to_string(),
    first_name: "John".to_string(),
    last_name: "Doe".to_string()
  }
}
