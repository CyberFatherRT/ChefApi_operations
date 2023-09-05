mod operations;

use actix_web::web::post;
use actix_web::{
    http::StatusCode, middleware::Logger, post, web::Path, App, HttpResponse, HttpServer,
};
use operations::*;
use utils::{Operation, Operations};

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
async fn cipher_handler(body: String, name: Path<Operations>) -> HttpResponse {
    match name.into_inner() {
        Operations::A1Z26CipherDecode => http_response(A1Z26CipherDecode, body),
        Operations::A1Z26CipherEncode => http_response(A1Z26CipherEncode, body),
        Operations::Adler32CheckSum => http_response(Adler32CheckSum, body),
        Operations::AffineCipherDecode => http_response(AffineCipherDecode, body),
        Operations::AffineCipherEncode => http_response(AffineCipherEncode, body),
        Operations::AnalyseHash => http_response(AnalyseHash, body),
        Operations::Argon2Compare => http_response(Argon2Compare, body),
        Operations::Argon2 => http_response(Argon2, body),
        Operations::AtbashCipher => http_response(AtbashCipher, body),
        Operations::Bcrypt => http_response(Bcrypt, body),
        Operations::BcryptCompare => http_response(BcryptCompare, body),
        Operations::BcryptParse => http_response(BcryptParse, body),
        Operations::BifidCipherEncode => http_response(BifidCipherEncode, body),
        Operations::Blake2b => http_response(Blake2b, body),
        Operations::Blake2s => http_response(Blake2s, body),
        Operations::FromBase64 => http_response(FromBase64, body),
        Operations::FromBase => http_response(FromBase, body),
        Operations::HMAC => http_response(Hmac, body),
        Operations::MD2 => http_response(MD2, body),
        Operations::MD4 => http_response(MD4, body),
        Operations::MD5 => http_response(MD5, body),
        Operations::RSADecrypt => http_response(RSADecrypt, body),
        Operations::RSAEncrypt => http_response(RSAEncrypt, body),
        Operations::SHA1 => http_response(SHA1, body),
        Operations::SHA2 => http_response(SHA2, body),
        Operations::SHA3 => http_response(SHA3, body),
        Operations::ToBase64 => http_response(ToBase64, body),
        Operations::ToBase => http_response(ToBase, body),
        Operations::VigenereCipherDecode => http_response(VigenereCipherDecode, body),
        Operations::VigenereCipherEncode => http_response(VigenereCipherEncode, body),
    }
}

async fn cipher_help_handler(name: Path<Operations>) -> HttpResponse {
    let res = match name.into_inner() {
        Operations::A1Z26CipherDecode => A1Z26CipherDecodeInfo::info(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncodeInfo::info(),
        Operations::Adler32CheckSum => Adler32CheckSumInfo::info(),
        Operations::AffineCipherDecode => AffineCipherDecodeInfo::info(),
        Operations::AffineCipherEncode => AffineCipherEncodeInfo::info(),
        Operations::AnalyseHash => AnalyseHashInfo::info(),
        Operations::Argon2Compare => Argon2CompareInfo::info(),
        Operations::Argon2 => Argon2Info::info(),
        Operations::AtbashCipher => AtbashCipherInfo::info(),
        Operations::Bcrypt => BcryptInfo::info(),
        Operations::BcryptCompare => BcryptCompareInfo::info(),
        Operations::BcryptParse => BcryptParseInfo::info(),
        Operations::BifidCipherEncode => BifidCipherEncodeInfo::info(),
        Operations::Blake2b => Blake2bInfo::info(),
        Operations::Blake2s => Blake2sInfo::info(),
        Operations::FromBase64 => FromBase64Info::info(),
        Operations::FromBase => FromBaseInfo::info(),
        Operations::HMAC => HmacInfo::info(),
        Operations::MD2 => Md2Info::info(),
        Operations::MD4 => Md4Info::info(),
        Operations::MD5 => Md5Info::info(),
        Operations::RSADecrypt => RSADecryptInfo::info(),
        Operations::RSAEncrypt => RSAEncryptInfo::info(),
        Operations::SHA1 => Sha1Info::info(),
        Operations::SHA2 => Sha2Info::info(),
        Operations::SHA3 => Sha3Info::info(),
        Operations::ToBase64 => ToBase64Info::info(),
        Operations::ToBase => ToBaseInfo::info(),
        Operations::VigenereCipherDecode => VigenereCipherDecodeInfo::info(),
        Operations::VigenereCipherEncode => VigenereCipherEncodeInfo::info(),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(res)
}
