use actix_web;
use async_graphql;

struct Query;

#[async_graphql::Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}


#[actix_web::get("/demo")]
async fn demo() -> impl actix_web::Responder {
    let schema = async_graphql::Schema::new(Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription);
    let res = schema.execute("{ add(a: 10, b: 20) }").await;

    actix_web::web::Json(res)
}


#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!")
}



#[actix_web::post("/echo")]
async fn echo(req_body: String) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body(req_body)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(demo)
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
