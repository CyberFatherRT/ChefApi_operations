pub mod libs;
mod macros;
mod operations;
pub mod traits;
pub mod utils;

pub use ciphers::operations::affine_cipher_encode_mod::{
    AffineCipherEncode, AffineCipherEncodeInfo,
};
pub use ciphers::operations::atbash_cipher_mod::{AtbashCipher, AtbashCipherInfo};
pub use ciphers::operations::bifid_cipher_encode_mod::{BifidCipherEncode, BifidCipherEncodeInfo};
pub use operations::ciphers::rsa_decode_mod::{RSADecrypt, RSADecryptInfo};
pub use operations::ciphers::rsa_encode_mod::{RSAEncrypt, RSAEncryptInfo};
pub use operations::ciphers::vigenere_cipher_decode_mod::{
    VigenereCipherDecode, VigenereCipherDecodeInfo,
};
pub use operations::ciphers::vigenere_cipher_encode_mod::{
    VigenereCipherEncode, VigenereCipherEncodeInfo,
};

// endregion

use crate::traits::StringTrait;
use serde::{Deserialize, Serialize};

pub const DOCS_URL: &str = "soon I transfer all documentation to somewhere :/";

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
    Adler32CheckSum,
    AffineCipherDecode,
    AffineCipherEncode,
    AnalyseHash,
    Argon2Compare,
    Argon2,
    AtbashCipher,
    Bcrypt,
    BcryptCompare,
    BcryptParse,
    BifidCipherEncode,
    Blake2b,
    Blake2s,
    FromBase64,
    FromBase,
    HMAC,
    MD2,
    MD4,
    MD5,
    RSADecrypt,
    RSAEncrypt,
    SHA1,
    SHA2,
    SHA3,
    ToBase64,
    ToBase,
    VigenereCipherDecode,
    VigenereCipherEncode,
}
