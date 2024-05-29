use rocket::get; 
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
  Template::render("home", context! { fields: "value" })
}
