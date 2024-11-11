mod app;
mod config;
mod db;
mod error;
mod routes;

use actix_files::Files;
use actix_web::*;
use config::AppConfig;
use db::DB;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new().await?;
    let addr = config.leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    println!("listening on http://{}", &addr);

    if let Err(err) = db::setup_database().await {
        eprintln!("Failed to set up database: {:?}", err);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Database setup failed",
        ));
    }

    HttpServer::new(move || {
        let leptos_options = &config.leptos_options;
        let site_root = &leptos_options.site_root;
        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .service(favicon)
            .configure(routes::config)
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                app::App,
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: web::Data<LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
