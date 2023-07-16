use actix_web::{
    web::{post, resource, Json, ServiceConfig},
    HttpResponse,
};

use serde::{Deserialize, Serialize};

pub mod operations;

#[derive(Serialize, Deserialize)]
pub struct Request {
    method: String,
    input: String,
}

pub async fn root(req: Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(req.0)
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api").route(post().to(root)));
}
