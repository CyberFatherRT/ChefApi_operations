use actix_web::HttpResponse;
use actix_web::web::{resource, post, Json, ServiceConfig};

use serde::Serialize;
use ciphers::error::Error;
use ciphers::*;

#[derive(Serialize)]
struct Response {
    output: Result<String, Error>
}

impl Response {
    fn new(output: Result<String, Error>) -> Self {
        Self { output }
    }
}

async fn ciphers_handler(request: Json<Request>) -> HttpResponse {

    let (lang, params, input) = (request.lang.to_owned(), request.params.to_owned(), request.input.to_owned());

    let response = match request.name {
        Operations::A1Z26CipherDecode => A1Z26CipherDecode::new(lang, params, input).run(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncode::new(lang, params, input).run(),
        Operations::AffineCipherDecode => AffineCipherDecode::new(lang, params, input).run(),
        Operations::AffineCipherEncode => AffineCipherEncode::new(lang, params, input).run(),
        Operations::FromBase64 => FromBase64::new(lang, params, input).run(),
        Operations::ToBase64 => ToBase64::new(lang, params, input).run(),
        Operations::VigenereCipherDecode => VigenereCipherDecode::new(lang, params, input).run(),
        Operations::VigenereCipherEncode => VigenereCipherEncode::new(lang, params, input).run(),
    };

    HttpResponse::Ok().json(Response::new(response))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api/ciphers").route(post().to(ciphers_handler)));
}