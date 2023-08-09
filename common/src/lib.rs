pub mod error;
mod macros;
pub mod traits;
pub mod utils;

use error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    AffineCipherDecode,
    AffineCipherEncode,
    FromBase64,
    ToBase64,
    VigenereCipherDecode,
    VigenereCipherEncode,
}

impl Operations {
    pub fn create_vec() -> Vec<Operations> {
        vec![Operations::VigenereCipherEncode]
    }
}

pub trait Operation {
    fn new(lang: String, params: Vec<String>, input: String) -> Self;
    fn run(&self) -> Result<String, Error>;
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub output: Result<String, Error>,
}

impl Response {
    pub fn new(output: Result<String, Error>) -> Self {
        Self { output }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub name: Operations,
    pub lang: String,
    pub params: Vec<String>,
    pub input: String,
}

impl Request {
    pub fn new(name: Operations, lang: String, params: Vec<String>, input: String) -> Self {
        Self {
            name,
            lang,
            params,
            input,
        }
    }
}
