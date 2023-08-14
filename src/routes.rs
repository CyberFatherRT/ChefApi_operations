use actix_web::{
    http::StatusCode,
    web::{post, resource, Path, ServiceConfig},
    HttpResponse,
};
use ciphers::*;
use serde::{Deserialize, Serialize};

fn http_response<'a, I, O, T: Operation<'a, I, O>>(structure: T, body: String) -> HttpResponse
where
    I: Deserialize<'a>,
    O: Serialize,
{
    let data = structure.run(&body);
    HttpResponse::build(if data.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    })
    .append_header(("Access-Control-Allow-Origin", "*"))
    .json(data)
}

async fn ciphers_handler(body: String, name: Path<Operations>) -> HttpResponse {
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
        Operations::FromBase64 => http_response(FromBase64, body),
        Operations::RSAEncrypt => http_response(RSAEncrypt, body),
        Operations::ToBase64 => http_response(ToBase64, body),
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
        Operations::FromBase64 => FromBase64Info::info(),
        Operations::RSAEncrypt => RSAEncryptInfo::info(),
        Operations::ToBase64 => ToBase64Info::info(),
        Operations::VigenereCipherDecode => VigenereCipherDecodeInfo::info(),
        Operations::VigenereCipherEncode => VigenereCipherEncodeInfo::info(),
    };
    HttpResponse::build(StatusCode::OK)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .content_type("application/json")
        .body(res)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api/{name}").route(post().to(ciphers_handler)))
        .service(resource("/api/{name}/help").route(post().to(cipher_help_handler)))
        .service(resource("/api/{name}/info").route(post().to(cipher_help_handler)));
}
