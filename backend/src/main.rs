mod config;
mod routes;

use actix_web::{middleware::Logger , App, HttpServer};
use actix_files::NamedFile;

use routes::configure;


async fn react_index() -> NamedFile{
    NamedFile::open("/frontend/dist/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .configure(configure)
            .service(actix_files::Files::new("/","./static_content").index_file("index.html"))
    })
    .bind(config::HOSTNAME)?
    .run()
    .await
}
