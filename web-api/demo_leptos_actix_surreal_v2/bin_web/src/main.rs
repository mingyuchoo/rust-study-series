#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use bin_web::app::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use log::info;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting server...");

    init_database().await?;

    let conf = get_configuration(None).await
                                      .unwrap();
    let addr = conf.leptos_options
                   .site_addr;
    let routes = generate_route_list(App);

    info!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new().service(Files::new("/pkg", format!("{site_root}/pkg")))
                  .service(Files::new("/assets", site_root))
                  .service(favicon)
                  .leptos_routes(leptos_options.to_owned(),
                                 routes.to_owned(),
                                 App)
                  .app_data(web::Data::new(leptos_options.to_owned()))
                  .wrap(middleware::Compress::default())
    }).bind(&addr)?
      .run()
      .await
}

#[cfg(feature = "ssr")]
async fn init_database() -> std::io::Result<()> {
    use lib_repo::setup_database;
    use log::error;

    if let Err(err) = setup_database().await {
        error!("Failed to set up database: {:?}", err);
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                       "Database setup failed"));
    }
    Ok(())
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(leptos_options: actix_web::web::Data<leptos::LeptosOptions>)
                 -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ),)?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use bin_web::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
