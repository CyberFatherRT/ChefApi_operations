#![allow(non_snake_case, dead_code)]

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok().body("I develop this site, don't touch me!!!")
                }),
            )
            .configure(api::configure)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
