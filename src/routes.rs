use actix_web::http::StatusCode;
use actix_web::web::{self, resource, ServiceConfig};
use actix_web::HttpResponse;
use ciphers::*;
use common::{Operation, Operations, Request, Response};

async fn ciphers_handler(request: String) -> HttpResponse {
    let request: Request = serde_json::from_str(&request).unwrap();

    let (lang, params, input) = (
        request.lang.to_owned(),
        request.params.to_owned(),
        request.input.to_owned(),
    );

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

    HttpResponse::build(StatusCode::OK)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(Response::new(response))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api/ciphers").route(web::post().to(ciphers_handler)));
}
