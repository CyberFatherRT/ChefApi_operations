pub mod libs;
mod macros;
mod operations;
pub mod traits;
pub mod utils;

pub use operations::a1z26_cipher_decode_mod::{A1Z26CipherDecode, A1Z26CipherDecodeInfo};
pub use operations::a1z26_cipher_encode_mod::{A1Z26CipherEncode, A1Z26CipherEncodeInfo};
pub use operations::adler32_checksum_mod::{Adler32CheckSum, Adler32CheckSumInfo};
pub use operations::affine_cipher_decode_mod::{AffineCipherDecode, AffineCipherDecodeInfo};
pub use operations::affine_cipher_encode_mod::{AffineCipherEncode, AffineCipherEncodeInfo};
pub use operations::analyse_hash_mod::{AnalyseHash, AnalyseHashInfo};
pub use operations::argon2_compare_mod::{Argon2Compare, Argon2CompareInfo};
pub use operations::argon2_mod::{Argon2, Argon2Info};
pub use operations::atbash_cipher_mod::{AtbashCipher, AtbashCipherInfo};
pub use operations::bcrypt_compare_mod::{BcryptCompare, BcryptCompareInfo};
pub use operations::bcrypt_mod::{Bcrypt, BcryptInfo};
pub use operations::bcrypt_parse_mod::{BcryptParse, BcryptParseInfo};
pub use operations::bifid_cipher_encode_mod::{BifidCipherEncode, BifidCipherEncodeInfo};
pub use operations::from_base64_mod::{FromBase64, FromBase64Info};
pub use operations::md2_mod::{MD2Info, MD2};
pub use operations::md4_mod::{MD4Info, MD4};
pub use operations::md5_mod::{MD5Info, MD5};
pub use operations::rsa_decode_mod::{RSADecrypt, RSADecryptInfo};
pub use operations::rsa_encode_mod::{RSAEncrypt, RSAEncryptInfo};
pub use operations::to_base64_mod::{ToBase64, ToBase64Info};
pub use operations::vigenere_cipher_decode_mod::{VigenereCipherDecode, VigenereCipherDecodeInfo};
pub use operations::vigenere_cipher_encode_mod::{VigenereCipherEncode, VigenereCipherEncodeInfo};

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
    FromBase64,
    MD2,
    MD4,
    MD5,
    RSADecrypt,
    RSAEncrypt,
    ToBase64,
    VigenereCipherDecode,
    VigenereCipherEncode,
}
