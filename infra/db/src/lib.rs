pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Error connecting to the database: {0}")]
    ConnectionError(#[from] PoolError),

    #[error("Error executing query: {0}")]
    QueryError(#[from] diesel::result::Error),
}

pub fn establish_connection() -> Result<PgPool, DatabaseError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(DatabaseError::ConnectionError)
}

pub fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, DatabaseError> {
    pool.get().map_err(DatabaseError::ConnectionError)
}
