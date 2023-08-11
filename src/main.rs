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

#[post("/api/{name}")]
async fn ciphers_handler(body: String, name: Path<Operations>) -> HttpResponse {
    let response = match name.into_inner() {
        Operations::Argon2 => Argon2::new(body).run(),
        _ => unreachable!(),
    };

    let status_code = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };

    HttpResponse::build(status_code)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(response)
}

// TODO: Make this function return info about operations.
async fn ciphers_info_handler(name: Path<Operations>) -> HttpResponse {
    HttpResponse::build(StatusCode::IM_A_TEAPOT).body("")
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
