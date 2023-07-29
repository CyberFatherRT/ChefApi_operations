mod config;
mod routes;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};

use routes::configure;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("I think I want new structure for web site")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(index)
            .configure(configure)
    })
    .bind(config::HOSTNAME)?
    .run()
    .await
}
