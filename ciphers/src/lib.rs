#![allow(dead_code)]

pub mod libs;
mod macros;
mod operations;
pub mod traits;
pub mod utils;

pub use operations::a1z26_cipher_decode_mod::A1Z26CipherDecode;
pub use operations::a1z26_cipher_encode_mod::A1Z26CipherEncode;
pub use operations::affine_cipher_decode_mod::AffineCipherDecode;
pub use operations::affine_cipher_encode_mod::AffineCipherEncode;
pub use operations::analyse_hash_mod::AnalyseHash;
pub use operations::argon2_compare_mod::Argon2Compare;
pub use operations::argon2_mod::Argon2;
pub use operations::atbash_cipher_mod::AtbashCipher;
pub use operations::bcrypt_compare_mod::BcryptCompare;
pub use operations::bcrypt_mod::Bcrypt;
pub use operations::from_base64_mod::FromBase64;
pub use operations::to_base64_mod::ToBase64;
pub use operations::vigenere_cipher_decode_mod::VigenereCipherDecode;
pub use operations::vigenere_cipher_encode_mod::VigenereCipherEncode;

use crate::traits::StringTrait;
use serde::{Deserialize, Serialize};

const DOCS_URL: &str = "soon I transfer all documentation to somewhere :/";

pub trait Operation<'a, I, O>
where
    I: Deserialize<'a>,
    O: Serialize,
{
    fn run(&self, request: &str) -> Result<O, String>;
    fn validate(&self, request: &'a str) -> Result<I, String> {
        self.deserialize(request)
    }

    fn deserialize(&self, request: &'a str) -> Result<I, String> {
        serde_json::from_str(request).map_err(|err| match err.to_string() {
            err if err.starts_with("unknown")
                || err.starts_with("missing")
                || err.starts_with("invalid") =>
            {
                err.split(" at line ")
                    .next()
                    .unwrap()
                    .to_string()
                    .capitalize()
                    + "."
            }
            err => err.capitalize() + ".",
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    AffineCipherDecode,
    AffineCipherEncode,
    AnalyseHash,
    Argon2Compare,
    Argon2,
    AtbashCipher,
    Bcrypt,
    BcryptCompare,
    FromBase64,
    ToBase64,
    VigenereCipherDecode,
    VigenereCipherEncode,
}
