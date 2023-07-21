#![feature(let_chains)]
#![allow(non_snake_case)]

use actix_web::{
    get, http::StatusCode, middleware::Logger, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use env_logger::Env;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let IP = std::env::var("IP").expect("IP must be set!");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(root)
            .configure(api::configure)
    })
    .bind((IP, 45678))?
    .run()
    .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=UTF-8")
        .body("<div style=\"text-align:center;margin-top:150px;font-weight:bolder;background-color:red;\">I do API and I DON'T CARE about frontend!</div>")
}
