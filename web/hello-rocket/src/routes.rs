use rocket::{routes, Build, Rocket};
use rocket_dyn_templates::Template;

pub fn build() -> Rocket<Build> {
    use crate::controllers;
    rocket::build().attach(Template::fairing()).mount(
        "/posts",
        routes![controllers::posts::post, controllers::posts::get],
    )
}
