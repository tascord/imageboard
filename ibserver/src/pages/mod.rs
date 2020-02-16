use actix_web::web;

pub mod posts;
pub mod tags;
pub mod search;
pub mod error;

// configure routing for our app
pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
    web::resource("/posts") // a service that will handle '/posts'
        .route(web::get().to(posts::get_handler))
        .route(web::post().to(posts::post_handler))
        .route(web::put().to(posts::put_handler))
        .route(web::delete().to(posts::delete_handler))
	)

    .service( // a service that will handle '/search'
    web::resource("/search")
    	.route(web::get().to(search::get_handler))
    )

    .service( // a service that will handle '/tags'
    web::resource("/tags")
    	.route(web::get().to(tags::get_handler)) // route a handler for get verb
    );
}