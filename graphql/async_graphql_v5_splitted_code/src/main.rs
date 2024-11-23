mod graphql_schema;
use crate::graphql_schema::{create_schema, Schema};

async fn graphiql() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            async_graphql::http::GraphiQLSource::build()
                .endpoint("http://localhost:4000")
                .subscription_endpoint("ws://localhost:4000/ws")
                .finish(),
        )
}

async fn graphql(schema: actix_web::web::Data<Schema>,
                 req: async_graphql_actix_web::GraphQLRequest)
                 -> async_graphql_actix_web::GraphQLResponse {
    schema.execute(req.into_inner())
          .await
          .into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new().app_data(actix_web::web::Data::new(create_schema()))
                             .route("/", actix_web::web::get().to(graphiql))
                             .route("/", actix_web::web::post().to(graphql))
    }).bind("127.0.0.1:4000")?
      .run()
      .await
}
