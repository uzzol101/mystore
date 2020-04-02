use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, PoolError,ConnectionManager, PooledConnection};


pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection () -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("database url not set");
   
    init_pool(&db_url).expect("Failed to create pool")
} 