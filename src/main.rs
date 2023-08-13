use actix_web::{middleware::Logger, App, HttpServer};

mod config;
mod routes;

use routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).configure(config)
    })
    .bind(config::HOSTNAME)?
    .run()
    .await
}
