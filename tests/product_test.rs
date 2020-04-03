extern crate mystore;

mod common;
use mystore::model::product::{Person, Product, NewProduct};
use mystore::routes::product::{init_routes, test};

use common::db::establish_connection;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
        use super::*;
        use actix_web::{test, web, App};

        #[actix_rt::test]
        async fn create_product() {
                let mut app = test::init_service(
                        App::new()
                                .data(establish_connection())
                                .configure(init_routes)

                ).await;
                let payload = NewProduct{
                        name: "test_product".to_string(),
                        stock: 100.20,
                        price: Some(400),
                };      
                let req = test::TestRequest::post()
                        .uri("/products")
                        .set_json(&payload)
                        .to_request();
                let resp: Product = test::read_response_json(&mut app, req).await;  
                println!("create product resp {:?}", resp);   
                assert_eq!(resp.name, "test_product");     
        }
        #[actix_rt::test]
        async fn get_product_list() {
                let mut app = test::init_service(
                        App::new()
                                .data(establish_connection())
                                .configure(init_routes),
                )
                .await;
                let req = test::TestRequest::get().uri("/products").to_request();
                let resp: Vec<Product> = test::read_response_json(&mut app, req).await;

                assert_eq!(resp.len(), 5);
                
               
        }
}
