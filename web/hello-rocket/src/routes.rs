use rocket::{routes, Build, Rocket};

pub fn build() -> Rocket<Build> {
    use crate::controllers;

    rocket::build()
        .mount("/", routes![
            controllers::health::health,
        ])
        .mount("/api", routes![
            controllers::posts::post,
            controllers::posts::get,
        ])
}
