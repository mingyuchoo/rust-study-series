use rocket::get; 
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
  Template::render("home/index", context! { first_name: "John", last_name: "Doe" })
}
