use rocket::fs::FileServer;
use rocket::{routes, Build, Rocket};

pub fn build() -> Rocket<Build> {
    use crate::controllers;

    rocket::build().mount("/public", FileServer::from("public"))
                   .mount("/", routes![controllers::home::index])
                   .mount("/api",
                          routes![controllers::posts::post,
                                  controllers::posts::get,
                                  controllers::health::health])
}
