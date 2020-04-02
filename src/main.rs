#[macro_use]
extern crate diesel;

use actix_web::{web, get, HttpResponse, HttpServer, App};
use dotenv::dotenv;
use std::env;

async fn index () -> HttpResponse {
    HttpResponse::Ok().body("hello actix")
}

mod model;
mod routes;
mod schema;
mod db;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().data(db::establish_connection()).configure(routes::product::init_routes)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
