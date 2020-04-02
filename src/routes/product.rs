use actix_web::{get,post,put,delete, web, HttpResponse, HttpRequest};
use crate::model::product::{NewProduct, Product};
use serde_json::json;


use crate::db::{PgPool, PgPooledConnection};

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}



#[get("/hello")]
async fn hello () -> HttpResponse {
    println!("here is in hello");
    HttpResponse::Ok().body("hello from hello")
}
#[post("/products")]
async fn create_product(_req: HttpRequest, product: web::Json<NewProduct>, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = pg_pool_handler(pool).unwrap();
    let result = Product::create_product(product.into_inner(), pg_pool).unwrap();
    HttpResponse::Ok().json(result)
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_product);
    cfg.service(hello);
}