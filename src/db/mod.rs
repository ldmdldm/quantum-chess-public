use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

pub mod models;
mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Initialize a connection pool to the PostgreSQL database
pub fn init_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");
    
    Ok(pool)
}

/// Get a connection from the pool
pub fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, r2d2::Error> {
    pool.get()
}

/// A shared database connection pool that can be cloned and passed to Actix handlers
#[derive(Clone)]
pub struct DbPool {
    pub pool: Arc<PgPool>,
}

impl DbPool {
    pub fn new(pool: PgPool) -> Self {
        DbPool {
            pool: Arc::new(pool),
        }
    }
    
    pub fn get_connection(&self) -> Result<PgPooledConnection, r2d2::Error> {
        self.pool.get()
    }
}

