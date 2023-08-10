#![allow(non_snake_case)]

mod config;

use actix_web::{
    http::StatusCode, middleware::Logger, post, web::Path, App, HttpResponse, HttpServer,
};
use ciphers::*;
use common::Operations;

#[post("/api/{name}")]
async fn ciphers_handler(body: String, name: Path<Operations>) -> HttpResponse {
    let response = match name.into_inner() {
        Operations::Argon2 => Argon2::new(body).run(),
        _ => unreachable!(),
    };

    HttpResponse::build(StatusCode::OK)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(response)
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
