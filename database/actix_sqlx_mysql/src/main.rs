use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use sqlx::MySqlPool;

#[get("/")]
async fn index() -> impl Responder {
    let connection =
        MySqlPool::connect("mysql://postgres:postgres@localhost:3306/postgres?prefer_socket=false")
            .await
            .unwrap();
    let mut pool = connection
        .try_acquire()
        .unwrap();

    let result =
        sqlx::query("INSERT INTO members(id, name) VALUES (fn_get_seq_8('MEMBER'), 'Tom')")
            .execute(&mut pool)
            .await;
    let rows = sqlx::query("SELECT count(*) as count FROM members")
        .fetch_all(&mut pool)
        .await
        .unwrap();

    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
