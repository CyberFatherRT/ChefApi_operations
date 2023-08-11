extern crate core;

pub mod libs;
mod macros;
mod operations;
pub mod traits;
pub mod utils;

pub use operations::a1z26cipher_decode_mod::{A1Z26CipherDecode, A1Z26CipherDecodeInfo};
pub use operations::a1z26cipher_encode_mod::{A1Z26CipherEncode, A1Z26CipherEncodeInfo};
pub use operations::affine_cipher_decode_mod::{AffineCipherDecode, AffineCipherDecodeInfo};
pub use operations::affine_cipher_encode_mod::{AffineCipherEncode, AffineCipherEncodeInfo};
pub use operations::analyse_hash_mod::{AnalyseHash, AnalyseHashInfo};
pub use operations::argon2_mod::{Argon2, Argon2Info};
pub use operations::from_base64_mod::{FromBase64, FromBase64Info};

use crate::traits::StringTrait;
use serde::{Deserialize, Serialize};

const DOCS_URL: &str = "soon I transfer all documentation to somewhere :/";

pub trait Operation<'a, I, O>
where
    I: Deserialize<'a>,
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
    Argon2,
    FromBase64,
}
