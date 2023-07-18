mod A1Z26CipherDecode_mod;
mod A1Z26CipherEncode_mod;
mod AffineCipherDecode_mod;
mod VigenereDecode_mod;
mod VigenereEncode_mod;


use actix_web::web::Json;
use serde::Deserialize;

use crate::api::error::Error;

use A1Z26CipherDecode_mod::A1Z26CipherDecode;
use A1Z26CipherEncode_mod::A1Z26CipherEncode;
use VigenereDecode_mod::VigenereDecode;
use VigenereEncode_mod::VigenereEncode;

trait Operation {
    fn new(input: Request) -> Box<Self>;
    fn run(&self) -> Result<String, Error>;
}

#[derive(Deserialize)]
pub struct Request {
    pub name: Operations,
    pub lang: String,
    pub params: Vec<String>,
    pub input: String,
}

#[derive(Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    VigenereDecode,
    VigenereEncode,
}

pub fn do_magic(request: Json<Request>) -> String {
    let result = match request.name {
        Operations::A1Z26CipherDecode => A1Z26CipherDecode::new(
            request.0
        ).run(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncode::new(
            request.0
        ).run(),
        Operations::VigenereDecode => VigenereDecode::new(
            request.0
        ).run(),
        Operations::VigenereEncode => VigenereEncode::new(
            request.0
        ).run(),
    };

    match result {
        Ok(s) => s,
        Err(e) => e.to_string()
    }
}