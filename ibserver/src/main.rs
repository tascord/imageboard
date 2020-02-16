#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate serde_json;

use actix_web::{App, HttpServer};
use actix_web::middleware::{Logger};

use env_logger;
use dotenv;

mod pages;
mod database;

#[actix_rt::main]
async fn main () -> std::io::Result<()> {
	dotenv::dotenv().ok(); 	// load a dotenv file
	let args = ProgramArguments::fill(); // get our programs options (port, etc)
	env_logger::builder() // init our logger
		.format_timestamp_secs()
		.format_module_path(false)
		.init();

	let dbpool = database::create_pool();

	// create a new http server
	HttpServer::new(move || { 
			App::new() // create an app factory
				.data(dbpool.clone()) // add a clone of the database pool to our app
				.wrap(Logger::new("%a \"%r\" %s %Dms")) // give our app premade logger middleware
				.configure(pages::config_app) // configure the routes
		})
		.workers(args.workers) // set the number of workers
		.bind(args.address)? // attempt to bind the address
		.run() // run
		.await // await for it exit
}

// this should be expanded
struct ProgramArguments {
	workers: usize,
	address: String,
}

impl ProgramArguments {
	pub fn fill () -> ProgramArguments {
		ProgramArguments {
			// get the numbers of workers from the env, or set to 8
			workers: match std::env::var("SERVER_MAX_WORKERS") {
				Ok(v) => v.parse::<usize>().expect("invalid worker count"),
				Err(_) => panic!("SERVER_MAX_WORKERS must be set")
			},
			// get the address from an enviroment var or panic
			address: match std::env::var("SERVER_ADDR") {
				Ok(v) => v,
				Err(_) => panic!("SERVER_ADDR must be set")
			}
		}
	}
}