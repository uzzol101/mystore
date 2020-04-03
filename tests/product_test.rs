extern crate mystore;

mod common;
use mystore::routes::product::{init_routes, test};
use mystore::model::product::{Product, Person};

use serde::{Deserialize, Serialize};

use common::db::{establish_connection};

#[derive(Debug,Deserialize, Serialize)]
struct AppState{
        count: i32
}



#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, HttpResponse};

//     #[actix_rt::test]
//     async fn test_index_get() {
//         let mut app = test::init_service(
//             App::new()
//                 .data(establish_connection())
//                 .configure(init_routes),
//         ).await;
//         let req = test::TestRequest::get().uri("/products").to_request();

        
//         let mut resp = test::call_service(&mut app, req).await;
//         assert!(resp.status().is_success());
//         println!("here is product {:?} ", resp);

//     }



// async fn index() -> HttpResponse {
//        HttpResponse::Ok().json(AppState { count: 40 })
// }

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .data(establish_connection())
                .configure(init_routes)
        ).await;
        let req = test::TestRequest::get().uri("/products").to_request();
        let resp: Vec<Product> = test::read_response_json(&mut app, req).await;
        
        // assert_eq!(resp.count, 4);
        println!("here is product {:?} ", resp);
    }
}
}