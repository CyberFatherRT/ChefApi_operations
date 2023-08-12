mod config;

use actix_web::{http::StatusCode, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use ciphers::*;
use serde::{Deserialize, Serialize};

// black magic with generic
fn http_response<'a, I, O, T: Operation<'a, I, O>>(structure: T, body: String) -> HttpResponse
where
    I: Deserialize<'a>,
    O: Serialize,
{
    let data = structure.run(&body);
    HttpResponse::build(if data.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    })
    .append_header(("Access-Control-Allow-Origin", "*"))
    .json(data)
}

#[post("/api/{name}")]
async fn ciphers_handler(body: String, name: web::Path<Operations>) -> HttpResponse {
    match name.into_inner() {
        Operations::A1Z26CipherDecode => http_response(A1Z26CipherDecode, body),
        Operations::A1Z26CipherEncode => http_response(A1Z26CipherEncode, body),
        Operations::AffineCipherDecode => http_response(AffineCipherDecode, body),
        Operations::AffineCipherEncode => http_response(AffineCipherEncode, body),
        Operations::AnalyseHash => http_response(AnalyseHash, body),
        Operations::Argon2Compare => http_response(Argon2Compare, body),
        Operations::Argon2 => http_response(Argon2, body),
        Operations::AtbashCipher => http_response(AtbashCipher, body),
        Operations::Bcrypt => http_response(Bcrypt, body),
        Operations::FromBase64 => http_response(FromBase64, body),
        Operations::ToBase64 => http_response(ToBase64, body),
        Operations::VigenereCipherDecode => http_response(VigenereCipherDecode, body),
        Operations::VigenereCipherEncode => http_response(VigenereCipherEncode, body),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).service(ciphers_handler)
    })
    .bind(config::HOSTNAME)?
    .run()
    .await
}
