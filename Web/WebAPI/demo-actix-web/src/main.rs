use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[get("/")]
async fn hello() -> impl Responder { HttpResponse::Ok().body("Hello world!") }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder { HttpResponse::Ok().body(req_body) }

async fn manual_hello() -> impl Responder { HttpResponse::Ok().body("Hey there!") }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Starting server on port {}", args.port);

    HttpServer::new(|| App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello)))
        .bind(("0.0.0.0", args.port))?
        .run()
        .await
}
