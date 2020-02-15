use actix_web::{web, App, Responder, HttpServer, middleware::Logger};
use env_logger;
use dotenv;

// our main function
#[actix_rt::main]
async fn main () -> std::io::Result<()> {
	// load a dotenv file
	dotenv::dotenv().ok();

	// get our programs options (port, etc)
	let args = ProgramArguments::fill();

	// init our logger
	env_logger::init();

	// create a new http server
	HttpServer::new(|| { 
			App::new() // create an app factory
				.wrap(Logger::default()) // give our app premade logger middleware
				.configure(config_app) // configure the routes
				.default_service(web::to(not_found_handler))
		})
		.workers(args.workers) // set the number of workers
		.bind(args.address)? // attempt to bind the address
		.run() // run
		.await // await for it exit
}

// this should be added to
#[allow(dead_code)]
struct ProgramArguments {
	workers: usize,
	address: String,
}

impl ProgramArguments {
	// we can fills these values using env or conf
	#[allow(dead_code)]
	pub fn fill () -> ProgramArguments {
		ProgramArguments {
			// get the numbers of workers from the env, or set to 8
			workers: match std::env::var("SERVER_MAX_WORKERS") {
				Ok(v) => v.parse::<usize>().expect("invalid worker count"),
				Err(_) => panic!("SERVER_MAX_WORKERS not set in enviroment variables")
			},
			// get the address from an enviroment var or panic
			address: match std::env::var("SERVER_ADDR") {
				Ok(v) => v,
				Err(_) => panic!("SERVER_ADDR not set in enviroment variables")
			}
		}
	}
}

// configure routing for our app
fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
    web::resource("/posts") // a service that will handle '/posts'
        .route(web::get().to(post_get_handler))
        .route(web::post().to(post_post_handler))
        .route(web::put().to(post_put_handler))
        .route(web::delete().to(post_delete_handler))
    )
    .service( // a service that will handle '/search'
    web::resource("/search")
    	.route(web::get().to(search_get_handler))
    )
    .service( // a service that will handle '/tags'
    web::resource("/tags")
    	.route(web::get().to(tag_get_handler)) // route a handler for get verb
    );
}

// TEST STUFF

use serde::Deserialize;
#[derive(Deserialize)]
struct PostQuery {
	id: u64,
}

// test handler stub
async fn post_get_handler(query: web::Query<PostQuery>) -> impl Responder {
	format!("post_post_handler {}", query.id)
}

async fn post_post_handler() -> impl Responder {"post_post_handler"}
async fn post_put_handler() -> impl Responder {"post_put_handler"}
async fn post_delete_handler() -> impl Responder {"post_delete_handler"}
async fn search_get_handler() -> impl Responder {"search_get_handler"}
async fn tag_get_handler() -> impl Responder {"tag_get_handler"}
async fn not_found_handler() -> impl Responder {"not_found_handler"}