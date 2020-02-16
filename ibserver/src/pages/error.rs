use actix_web::{HttpResponse, Error};

pub async fn not_found_handler() -> Result<HttpResponse, Error> {
	unimplemented!()
}