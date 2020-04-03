#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use actix_web::{web, get, HttpResponse, HttpServer, App};
use crate::db::{establish_connection};

pub mod routes;
pub mod model;
mod db;
mod schema;
mod helper;

