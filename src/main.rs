#[macro_use]
extern crate lazy_static;

mod data;
mod routes;
mod schema;

use std::io::Result;
use std::sync::Arc;

use actix_web::{ http, HttpServer, App, middleware };
use actix_cors::Cors;

use routes::register_routes;
use schema::create_schema;

#[actix_web::main]
async fn main() -> Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .wrap(
                Cors::new()
                  .allowed_origin("http://localhost:3000")
                  .allowed_origin("http://localhost:8080")
                  .allowed_methods(vec!["GET", "POST"])
                  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                  .allowed_header(http::header::CONTENT_TYPE)
                  .max_age(3600)
                  .finish()
            )
            .data(schema.clone())
            .configure(register_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}