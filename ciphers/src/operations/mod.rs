mod a1z26cipher_decode_mod;
mod a1z26cipher_encode_mod;
mod affine_cipher_decode_mod;
mod affine_cipher_encode_mod;
mod from_base64_mod;
mod to_base64_mod;
mod vigenere_cipher_decode_mod;
mod vigenere_cipher_encode_mod;

pub use a1z26cipher_decode_mod::A1Z26CipherDecode;
pub use a1z26cipher_encode_mod::A1Z26CipherEncode;
pub use affine_cipher_decode_mod::AffineCipherDecode;
pub use affine_cipher_encode_mod::AffineCipherEncode;
pub use from_base64_mod::FromBase64;
pub use to_base64_mod::ToBase64;
pub use vigenere_cipher_decode_mod::VigenereCipherDecode;
pub use vigenere_cipher_encode_mod::VigenereCipherEncode;

use crate::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
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

pub trait Operation {
    fn new(lang: String, params: Vec<String>, input: String) -> Self;
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
