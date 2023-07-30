mod config;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};

use routes::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).configure(configure)
    })
    .bind(config::HOSTNAME)?
    .run()
    .await
}
