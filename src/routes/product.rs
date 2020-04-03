use actix_web::{web, get, post,put, delete, HttpResponse};
use crate::model::product::{Product, NewProduct, Person};
use crate::db::{PgPool};
use crate::helper::db::{pool_handler};
use serde_json::json;

#[get("/test")]
pub async fn test() -> HttpResponse {
    HttpResponse::Ok().json(Person { name: String::from("uzzol"), age: 30 })
}

#[get("/products")]
pub async fn get_all_products(pool: web::Data<PgPool>) ->  HttpResponse {
    let conn = pool_handler(pool).unwrap();
    let products_list = Product::all_products(conn).unwrap();
    HttpResponse::Ok().json(products_list)
}

#[post("/products")]
async fn create_new_product(product: web::Json<NewProduct>, pool: web::Data<PgPool>) -> HttpResponse {
    let conn = pool_handler(pool).unwrap();
    let new_prod = Product::new_product(product.into_inner(), conn).unwrap();
    HttpResponse::Ok().json(new_prod)
}


#[put("/products/{id}")]
async fn update_product(product: web::Json<Product>, pool: web::Data<PgPool>) -> HttpResponse {
    let conn = pool_handler(pool).unwrap();
    let update_prod = Product::update_product(product.into_inner(), conn).unwrap();
    HttpResponse::Ok().json(update_prod)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_products);
    cfg.service(create_new_product);
    cfg.service(update_product);
    cfg.service(test);

}