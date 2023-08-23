mod operations;

use actix_web::web::post;
use actix_web::{
    http::StatusCode, middleware::Logger, post, web::Path, App, HttpResponse, HttpServer,
};
use operations::*;
use utils::{Operation, OperationCiphers};

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
async fn cipher_handler(body: String, name: Path<OperationCiphers>) -> HttpResponse {
    match name.into_inner() {
        OperationCiphers::A1Z26CipherDecode => http_response(A1Z26CipherDecode, body),
        OperationCiphers::A1Z26CipherEncode => http_response(A1Z26CipherEncode, body),
        OperationCiphers::AffineCipherDecode => http_response(AffineCipherDecode, body),
        OperationCiphers::AffineCipherEncode => http_response(AffineCipherEncode, body),
        OperationCiphers::AtbashCipher => http_response(AtbashCipher, body),
        OperationCiphers::BifidCipherEncode => http_response(BifidCipherEncode, body),
        OperationCiphers::RSADecrypt => http_response(RSADecrypt, body),
        OperationCiphers::RSAEncrypt => http_response(RSAEncrypt, body),
        OperationCiphers::VigenereCipherDecode => http_response(VigenereCipherDecode, body),
        OperationCiphers::VigenereCipherEncode => http_response(VigenereCipherEncode, body),
    }
}

async fn cipher_help_handler(name: Path<OperationCiphers>) -> HttpResponse {
    let res = match name.into_inner() {
        OperationCiphers::A1Z26CipherDecode => A1Z26CipherDecodeInfo::info(),
        OperationCiphers::A1Z26CipherEncode => A1Z26CipherEncodeInfo::info(),
        OperationCiphers::AffineCipherDecode => AffineCipherDecodeInfo::info(),
        OperationCiphers::AffineCipherEncode => AffineCipherEncodeInfo::info(),
        OperationCiphers::AtbashCipher => AtbashCipherInfo::info(),
        OperationCiphers::BifidCipherEncode => BifidCipherEncodeInfo::info(),
        OperationCiphers::RSADecrypt => RSADecryptInfo::info(),
        OperationCiphers::RSAEncrypt => RSAEncryptInfo::info(),
        OperationCiphers::VigenereCipherDecode => VigenereCipherDecodeInfo::info(),
        OperationCiphers::VigenereCipherEncode => VigenereCipherEncodeInfo::info(),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(res)
}
