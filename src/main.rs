#[macro_use]
extern crate diesel;
use crate::db::establish_connection;
use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
mod db;
mod helper;
mod model;
mod routes;
mod schema;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST","PUT","DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(establish_connection())
            .configure(routes::product::init_routes)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
