mod operations;

use actix_web::web::post;
use actix_web::{
    http::StatusCode, middleware::Logger, post, web::Path, App, HttpResponse, HttpServer,
};
use operations::*;
use utils::{Operation, OperationCrypto};

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

fn http_response<'a, I, O, T>(structure: T, body: String) -> HttpResponse
where
    I: serde::Deserialize<'a>,
    O: serde::Serialize,
    T: Operation<'a, I, O>,
{
    let response = structure.run(&body);
    let status_code = match response {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::IM_A_TEAPOT,
    };
    HttpResponse::build(status_code).json(response)
}

#[post("/{name}")]
async fn cipher_handler(body: String, name: Path<OperationCrypto>) -> HttpResponse {
    match name.into_inner() {
        OperationCrypto::Adler32CheckSum => http_response(Adler32CheckSum, body),
        OperationCrypto::Argon2Compare => http_response(Argon2Compare, body),
        OperationCrypto::Argon2 => http_response(Argon2, body),
        OperationCrypto::Bcrypt => http_response(Bcrypt, body),
        OperationCrypto::BcryptCompare => http_response(BcryptCompare, body),
        OperationCrypto::BcryptParse => http_response(BcryptParse, body),
        OperationCrypto::Hmac => http_response(Hmac, body),
    }
}

async fn cipher_help_handler(name: Path<OperationCrypto>) -> HttpResponse {
    let res = match name.into_inner() {
        OperationCrypto::Adler32CheckSum => Adler32CheckSumInfo::info(),
        OperationCrypto::Argon2Compare => Argon2CompareInfo::info(),
        OperationCrypto::Argon2 => Argon2Info::info(),
        OperationCrypto::Bcrypt => BcryptInfo::info(),
        OperationCrypto::BcryptCompare => BcryptCompareInfo::info(),
        OperationCrypto::BcryptParse => BcryptParseInfo::info(),
        OperationCrypto::Hmac => HmacInfo::info(),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(res)
}
