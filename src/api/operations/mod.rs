mod A1Z26CipherDecode_mod;
mod A1Z26CipherEncode_mod;

use actix_web::web::Json;
use serde::Deserialize;

use crate::api::error::Error;

use A1Z26CipherDecode_mod::A1Z26CipherDecode;
use A1Z26CipherEncode_mod::A1Z26CipherEncode;

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

#[derive(Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
}

pub fn do_magic(request: Json<Request>) -> String {
    let result = match request.name {
        Operations::A1Z26CipherDecode => A1Z26CipherDecode::new(
            request.0
        ).run(),
        Operations::A1Z26CipherEncode => A1Z26CipherEncode::new(
            request.0
        ).run(),
    };

    match result {
        Ok(s) => s,
        Err(e) => e.to_string()
    }
}