use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod models;
pub mod schema;

pub fn create_pool () -> DbPool {
	// get our database url from env var
	let database_url = std::env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");
		
	// create a connection manager for a postgres connection
	let manager = ConnectionManager::<PgConnection>::new(database_url);

	// build our pool
	r2d2::Pool::builder()
		.build(manager)
		.expect("Error creating database pool")
}