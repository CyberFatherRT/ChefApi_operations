mod error;
mod libs;
mod macros;
mod operations;
mod traits;
mod utils;

pub use operations::a1z26cipher_decode_mod::A1Z26CipherDecode;
pub use operations::a1z26cipher_encode_mod::A1Z26CipherEncode;
pub use operations::affine_cipher_decode_mod::AffineCipherDecode;
pub use operations::affine_cipher_encode_mod::AffineCipherEncode;
pub use operations::from_base64_mod::FromBase64;
pub use operations::to_base64_mod::ToBase64;
pub use operations::vigenere_cipher_decode_mod::VigenereCipherDecode;
pub use operations::vigenere_cipher_encode_mod::VigenereCipherEncode;

pub use libs::affine_trait::AffineCipher;
pub use libs::base64::{from_base64, to_base64};
pub use libs::vigenere_trait::VigenereCipher;

pub use error::Error;

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
    fn validate(&self) -> Result<(), Error>;
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
