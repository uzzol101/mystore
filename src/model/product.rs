use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::products;
use crate::db::{establish_connection, PgPooledConnection};


#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32> // For a value that can be null,                       
}

#[derive(Insertable, Deserialize)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>
}


impl Product {

    pub fn create_product(new_product: NewProduct, conn: PgPooledConnection) -> Result<Self, diesel::result::Error> {
       
        let result = diesel::insert_into(products::table).values(new_product).get_result(&conn).unwrap();

        Ok(result)
    }

 
}