mod A1Z26CipherDecode_mod;
mod A1Z26CipherEncode_mod;
mod AffineCipherDecode_mod;
mod VigenereDecode_mod;
mod VigenereEncode_mod;

use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::api::error::Error;

use A1Z26CipherDecode_mod::A1Z26CipherDecode;
use A1Z26CipherEncode_mod::A1Z26CipherEncode;
use AffineCipherDecode_mod::AffineCipherDecode;
use VigenereDecode_mod::VigenereDecode;
use VigenereEncode_mod::VigenereEncode;

#[derive(Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    AffineCipherDecode,
    VigenereDecode,
    VigenereEncode,
}

trait Operation {
    fn new(input: Request) -> Box<Self>;
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

pub fn do_magic(request: Json<Request>) -> Response {
    let result = match request.name {
        Operations::A1Z26CipherDecode => A1Z26CipherDecode::new(request.0).run(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncode::new(request.0).run(),
        Operations::AffineCipherDecode => AffineCipherDecode::new(request.0).run(),
        Operations::VigenereDecode => VigenereDecode::new(request.0).run(),
        Operations::VigenereEncode => VigenereEncode::new(request.0).run(),
    };

    Response::new(result)
}
