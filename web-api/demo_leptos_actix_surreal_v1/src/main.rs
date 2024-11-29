mod routes;

use actix_files::Files;
use memo_leptos_actix_surreal_v1::config::AppConfig;
use memo_leptos_actix_surreal_v1::db::setup_database;
use memo_leptos_actix_surreal_v1::web::App;
use actix_web::*;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use leptos_router::RouteListing;
use log::{error, info};
use routes::routes_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server...");

    init_database().await?;

    let app_config = AppConfig::new().await?;
    let addr = app_config.leptos_options
                         .site_addr;
    println!("listening on http://{}", &addr);

    start_server(app_config, generate_route_list(App)).await
}

pub async fn init_database() -> std::io::Result<()> {
    if let Err(err) = setup_database().await {
        error!("Failed to set up database: {:?}", err);
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                       "Database setup failed"));
    }
    Ok(())
}

async fn start_server(app_config: AppConfig,
                      routes: Vec<RouteListing>)
                      -> std::io::Result<()> {
    let addr = app_config.leptos_options
                         .site_addr;
    HttpServer::new(move || {
        let leptos_options = &app_config.leptos_options;
        let site_root = &leptos_options.site_root;
        App::new().service(Files::new("/pkg", format!("{site_root}/pkg")))
                  .service(Files::new("/assets", site_root))
                  .service(favicon)
                  .configure(routes_config)
                  .leptos_routes(leptos_options.to_owned(),
                                 routes.to_owned(),
                                 App)
                  .app_data(web::Data::new(leptos_options.to_owned()))
                  .wrap(middleware::Compress::default())
    }).bind(&addr)?
      .run()
      .await
}

#[actix_web::get("favicon.ico")]
async fn favicon(leptos_options: web::Data<LeptosOptions>)
                 -> actix_web::Result<actix_files::NamedFile> {
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}
