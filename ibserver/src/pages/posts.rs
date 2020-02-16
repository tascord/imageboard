use actix_web::{HttpResponse, Error};

pub async fn get_handler() -> Result<HttpResponse, Error> {
	Ok(HttpResponse::Ok().body("post::get_handler"))
}

pub async fn post_handler() -> Result<HttpResponse, Error> {
	unimplemented!()
}

pub async fn put_handler() -> Result<HttpResponse, Error> {
	unimplemented!()
}

pub async fn delete_handler() -> Result<HttpResponse, Error> {
	unimplemented!()
}