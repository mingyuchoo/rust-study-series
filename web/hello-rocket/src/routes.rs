use rocket::fs::{relative, FileServer};
use rocket::{routes, Build, Rocket};
use rocket_dyn_templates::Template;

pub fn build() -> Rocket<Build> {
    use crate::controllers;

    rocket::build()
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/", routes![controllers::home::index])
        .mount(
            "/api",
            routes![
                controllers::posts::post,
                controllers::posts::get,
                controllers::health::health
            ],
        )
        .attach(Template::fairing())
}
