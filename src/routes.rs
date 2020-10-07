use std::sync::Arc;

use actix_web::{ get, post, HttpResponse, web };

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::{ Schema };
use crate::data::get_products;

#[get("/")]
pub async fn landing() -> &'static str {
    "Hello World"
}

#[get("/data")]
pub async fn products() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(get_products())
}

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            graphiql_source("http://localhost:8080/graphql", None)
        )
}

#[post("/graphql")]
pub async fn graphql(
    schema: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>
) -> HttpResponse {
    let res = request.execute(&schema, &()).await;
    
    HttpResponse::Ok()
        .json(res)
}

pub fn register_routes(config: &mut web::ServiceConfig) {
    config.service(landing).service(products).service(graphiql).service(graphql);
}