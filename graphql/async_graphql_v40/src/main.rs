struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

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

async fn index(
    schema: actix_web::web::Data<
        async_graphql::Schema<
            QueryRoot,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        >,
    >,
    req: async_graphql_actix_web::GraphQLRequest,
) -> async_graphql_actix_web::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(async_graphql::Schema::new(
                QueryRoot,
                async_graphql::EmptyMutation,
                async_graphql::EmptySubscription,
            )))
            .service(
                actix_web::web::resource("/")
                    .guard(actix_web::guard::Get())
                    .to(graphiql),
            )
            .service(
                actix_web::web::resource("/")
                    .guard(actix_web::guard::Post())
                    .to(index),
            )
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
