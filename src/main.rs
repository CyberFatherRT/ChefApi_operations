mod config;

use actix_web::web::post;
use actix_web::{
    http::StatusCode,
    middleware::Logger,
    post,
    web::{resource, Path},
    App, HttpResponse, HttpServer,
};
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
async fn ciphers_handler(body: String, name: Path<Operations>) -> HttpResponse {
    match name.into_inner() {
        Operations::A1Z26CipherDecode => http_response(A1Z26CipherDecode, body),
        Operations::A1Z26CipherEncode => http_response(A1Z26CipherEncode, body),
        Operations::AffineCipherDecode => http_response(AffineCipherDecode, body),
        Operations::AffineCipherEncode => http_response(AffineCipherEncode, body),
        Operations::AnalyseHash => http_response(AnalyseHash, body),
        Operations::Argon2 => http_response(Argon2, body),
        Operations::FromBase64 => http_response(FromBase64, body),
    }
}

async fn ciphers_info_handler(name: Path<Operations>) -> HttpResponse {
    let response = match name.into_inner() {
        Operations::A1Z26CipherDecode => A1Z26CipherDecodeInfo::info(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncodeInfo::info(),
        Operations::AffineCipherDecode => AffineCipherDecodeInfo::info(),
        Operations::AffineCipherEncode => AffineCipherEncodeInfo::info(),
        Operations::AnalyseHash => AnalyseHashInfo::info(),
        Operations::Argon2 => Argon2Info::info(),
        Operations::FromBase64 => FromBase64Info::info(),
    };

    HttpResponse::build(StatusCode::OK)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .content_type("application/json")
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).service(ciphers_handler).service(
            resource(vec!["/api/{name}/help", "/api/{name}/info"])
                .route(post().to(ciphers_info_handler))
                .route(post().to(ciphers_info_handler)),
        )
    })
    .bind(config::HOSTNAME)?
    .run()
    .await
}
