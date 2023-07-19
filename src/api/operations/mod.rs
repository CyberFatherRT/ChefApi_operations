mod A1Z26CipherDecode_mod;
mod A1Z26CipherEncode_mod;
mod AffineCipherDecode_mod;
mod AffineCipherEncode_mod;
mod ToBase64_mod;
mod VigenereCipherDecode_mod;
mod VigenereCipherEncode_mod;

use A1Z26CipherDecode_mod::A1Z26CipherDecode;
use A1Z26CipherEncode_mod::A1Z26CipherEncode;
use AffineCipherDecode_mod::AffineCipherDecode;
use AffineCipherEncode_mod::AffineCipherEncode;
use ToBase64_mod::ToBase64;
use VigenereCipherDecode_mod::VigenereCipherDecode;
use VigenereCipherEncode_mod::VigenereCipherEncode;

use crate::api::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    AffineCipherDecode,
    AffineCipherEncode,
    ToBase64,
    VigenereCipherDecode,
    VigenereCipherEncode,
}

trait Operation {
    fn new(input: Request) -> Self;
    fn run(&self) -> Result<String, Error>;
    fn validate(&self) -> Result<(), Error>;
}

#[derive(Deserialize)]
pub struct Request {
    pub name: Operations,
    pub lang: String,
    pub params: Vec<String>,
    pub input: String,
}

#[derive(Serialize)]
pub struct Response {
    pub output: Result<String, Error>,
}

impl Response {
    fn new(output: Result<String, Error>) -> Self {
        Self { output }
    }
}

pub fn some_magic(request: actix_web::web::Json<Request>) -> Response {
    let result = match request.name {
        Operations::A1Z26CipherDecode => A1Z26CipherDecode::new(request.0).run(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncode::new(request.0).run(),
        Operations::AffineCipherDecode => AffineCipherDecode::new(request.0).run(),
        Operations::AffineCipherEncode => AffineCipherEncode::new(request.0).run(),
        Operations::ToBase64 => ToBase64::new(request.0).run(),
        Operations::VigenereCipherDecode => VigenereCipherDecode::new(request.0).run(),
        Operations::VigenereCipherEncode => VigenereCipherEncode::new(request.0).run(),
    };

    Response::new(result)
}
