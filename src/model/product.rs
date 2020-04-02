use diesel::prelude::*;
use crate::schema::products;
use serde::{Deserialize, Serialize};
use diesel::result::Error as DieselError;
use crate::db::{establish_connection, PgPooledConnection};
use actix_web::{web};
use crate::helper::db::{pool_handler};

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize, Queryable)]
#[table_name = "products"]
pub struct NewProduct {
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>,
}


impl Product {
    pub fn new_product(product: NewProduct, conn: PgPooledConnection) -> Result<Product, DieselError> {
        
        let product = diesel::insert_into(products::table).values(product).get_result::<Product>(&conn).unwrap();
        Ok(product)
    }

    pub fn update_product(product: Product, conn: PgPooledConnection) -> Result<Product, DieselError> {
     
      let updated_prod = diesel::update(products::table).filter(products::id.eq(product.id)).set(product).get_result::<Product>(&conn).unwrap();

      Ok(updated_prod)
    }

    pub fn all_products(conn: PgPooledConnection) -> Result<Vec<Product>, DieselError> {
    
      let products = products::table.load::<Product>(&conn).unwrap();
      Ok(products)
    }
}