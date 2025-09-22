use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web, middleware::Logger};
use actix_files as fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[get("/hello")]
async fn hello() -> impl Responder { 
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello from Rust backend!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder { HttpResponse::Ok().body(req_body) }

async fn manual_hello() -> impl Responder { HttpResponse::Ok().body("Hey there!") }

#[get("/data")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Data from Rust backend!",
        "items": [
            {
                "id": 1,
                "name": "Rust",
                "description": "A systems programming language focused on safety and performance"
            },
            {
                "id": 2,
                "name": "Actix-web",
                "description": "A powerful, pragmatic, and extremely fast web framework for Rust"
            },
            {
                "id": 3,
                "name": "React",
                "description": "A JavaScript library for building user interfaces"
            }
        ]
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Starting server on port {}", args.port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // API routes
            .service(
                web::scope("/api")
                    .service(hello)
                    .service(echo)
                    .service(get_data)
                    .route("/hey", web::get().to(manual_hello))
            )
            // Static files (frontend)
            .service(fs::Files::new("/", "./wwwroot").index_file("index.html"))
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
