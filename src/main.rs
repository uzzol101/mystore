#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use actix_web::{web, get, HttpResponse, HttpServer, App};
use crate::db::{establish_connection};

mod routes;
mod model;
mod db;
mod schema;
mod helper;


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix")
}

#[actix_rt::main]
async fn main () -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().data(establish_connection()).configure(routes::product::init_routes)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}