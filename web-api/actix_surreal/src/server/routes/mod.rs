mod person;
mod session;
mod token;

use actix_web::web;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(person::create_person)
       .service(person::read_person)
       .service(person::update_person)
       .service(person::delete_person)
       .service(person::list_people)
       .service(session::session)
       .service(token::make_new_user)
       .service(token::get_new_token);
}
