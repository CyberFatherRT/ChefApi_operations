mod operations;

use actix_web::web::post;
use actix_web::{
    http::StatusCode, middleware::Logger, post, web::Path, App, HttpResponse, HttpServer,
};
use operations::*;
use utils::{Operation, OperationsHashing};

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
async fn cipher_handler(body: String, name: Path<OperationsHashing>) -> HttpResponse {
    match name.into_inner() {
        OperationsHashing::AnalyseHash => http_response(AnalyseHash, body),
        OperationsHashing::Blake2b => http_response(Blake2b, body),
        OperationsHashing::Blake2s => http_response(Blake2s, body),
        OperationsHashing::MD2 => http_response(MD2, body),
        OperationsHashing::MD4 => http_response(MD4, body),
        OperationsHashing::MD5 => http_response(MD5, body),
        OperationsHashing::SHA1 => http_response(SHA1, body),
        OperationsHashing::SHA2 => http_response(SHA2, body),
        OperationsHashing::SHA3 => http_response(SHA3, body),
    }
}

async fn cipher_help_handler(name: Path<OperationsHashing>) -> HttpResponse {
    let res = match name.into_inner() {
        OperationsHashing::AnalyseHash => AnalyseHashInfo::info(),
        OperationsHashing::Blake2b => Blake2bInfo::info(),
        OperationsHashing::Blake2s => Blake2sInfo::info(),
        OperationsHashing::MD2 => Md2Info::info(),
        OperationsHashing::MD4 => Md4Info::info(),
        OperationsHashing::MD5 => Md5Info::info(),
        OperationsHashing::SHA1 => Sha1Info::info(),
        OperationsHashing::SHA2 => Sha2Info::info(),
        OperationsHashing::SHA3 => Sha3Info::info(),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(res)
}
