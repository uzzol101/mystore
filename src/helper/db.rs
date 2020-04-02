use crate::db::{PgPooledConnection, PgPool};
use actix_web::{web, HttpResponse};

pub fn pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool.get().map_err(|error|{HttpResponse::InternalServerError().json(error.to_string())})
}