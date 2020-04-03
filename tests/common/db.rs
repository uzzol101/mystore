use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager, PoolError};

pub type PgPool = Pool::<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


pub fn init_pool(db_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}


pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL_TEST").expect("database url not set");
    init_pool(&db_url).unwrap()
}

