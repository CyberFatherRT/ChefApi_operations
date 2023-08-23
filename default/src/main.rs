mod operations;

use actix_web::web::post;
use actix_web::{
    http::StatusCode, middleware::Logger, post, web::Path, App, HttpResponse, HttpServer,
};
use operations::*;
use utils::{Operation, OperationDefault};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(cipher_handler)
            .route("/{name}/help", post().to(cipher_help_handler))
            .route("/{name}/info", post().to(cipher_help_handler))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

fn http_response<'a, I, O, T: Operation<'a, I, O>>(structure: T, body: String) -> HttpResponse
where
    I: serde::Deserialize<'a>,
    O: serde::Serialize,
{
    let response = structure.run(&body);
    let status_code = match response {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::IM_A_TEAPOT,
    };
    HttpResponse::build(status_code).json(response)
}

#[post("/{name}")]
async fn cipher_handler(body: String, name: Path<OperationDefault>) -> HttpResponse {
    match name.into_inner() {
        OperationDefault::FromBase64 => http_response(FromBase64, body),
        OperationDefault::FromBase => http_response(FromBase, body),
        OperationDefault::ToBase64 => http_response(ToBase64, body),
        OperationDefault::ToBase => http_response(ToBase, body),
    }
}

async fn cipher_help_handler(name: Path<OperationDefault>) -> HttpResponse {
    let res = match name.into_inner() {
        OperationDefault::FromBase64 => FromBase64Info::info(),
        OperationDefault::FromBase => FromBaseInfo::info(),
        OperationDefault::ToBase64 => ToBase64Info::info(),
        OperationDefault::ToBase => ToBaseInfo::info(),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(res)
}
