use actix_web::{get, route, web, Error, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::{
    db::DBPool,
    schemas::root::{create_schema, Context, Schema},
};

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_schema()))
        .service(graphql)
        .service(graphql_playground);
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    pool: web::Data<DBPool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        db_pool: pool.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}