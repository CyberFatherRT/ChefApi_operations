#![feature(let_chains)]
#![allow(non_snake_case)]

use actix_web::{
    middleware::Logger, App, HttpServer,
};
use env_logger::Env;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(api::configure))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
