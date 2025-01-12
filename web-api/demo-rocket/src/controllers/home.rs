use askama::Template;
use rocket::get;

#[derive(Template)]
#[template(path = "home/index.html")]
pub struct IndexTemplate {
    pub title:      String,
    pub first_name: String,
    pub last_name:  String,
}

#[get("/")]
pub fn index() -> IndexTemplate {
    IndexTemplate {
        title:      "Index".to_string(),
        first_name: "John".to_string(),
        last_name:  "Doe".to_string(),
    }
}
