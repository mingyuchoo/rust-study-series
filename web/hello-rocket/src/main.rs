use rocket::{launch, routes};
use rocket_dyn_templates::Template;

/*
crate(main)
  |- schema
  |- models
  |- services
*/
mod models;
mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![services::create_post, services::index])
}
