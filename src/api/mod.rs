pub mod lib;
pub mod operations;

pub mod error;
pub mod macros;
pub mod utils;

use actix_web::{
    web::{post, resource, Json, ServiceConfig},
    HttpResponse,
};

use operations::{do_magic, Request};

pub async fn root(req: Json<Request>) -> HttpResponse {
    HttpResponse::Ok().body(do_magic(req))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api").route(post().to(root)));
}
