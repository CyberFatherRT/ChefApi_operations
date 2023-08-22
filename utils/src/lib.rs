pub mod libs;
mod macros;
mod operations;
pub mod traits;
pub mod utils;

// region ciphers

pub use operations::ciphers::a1z26_cipher_decode_mod::{A1Z26CipherDecode, A1Z26CipherDecodeInfo};
pub use operations::ciphers::a1z26_cipher_encode_mod::{A1Z26CipherEncode, A1Z26CipherEncodeInfo};
pub use operations::ciphers::affine_cipher_decode_mod::{
    AffineCipherDecode, AffineCipherDecodeInfo,
};
pub use operations::ciphers::affine_cipher_encode_mod::{
    AffineCipherEncode, AffineCipherEncodeInfo,
};
pub use operations::ciphers::atbash_cipher_mod::{AtbashCipher, AtbashCipherInfo};
pub use operations::ciphers::bifid_cipher_encode_mod::{BifidCipherEncode, BifidCipherEncodeInfo};
pub use operations::ciphers::rsa_decode_mod::{RSADecrypt, RSADecryptInfo};
pub use operations::ciphers::rsa_encode_mod::{RSAEncrypt, RSAEncryptInfo};
pub use operations::ciphers::vigenere_cipher_decode_mod::{
    VigenereCipherDecode, VigenereCipherDecodeInfo,
};
pub use operations::ciphers::vigenere_cipher_encode_mod::{
    VigenereCipherEncode, VigenereCipherEncodeInfo,
};

// endregion

// region crypto

pub use crypto::operations::argon2_compare_mod::{Argon2Compare, Argon2CompareInfo};
pub use crypto::operations::argon2_mod::{Argon2, Argon2Info};
pub use crypto::operations::bcrypt_compare_mod::{BcryptCompare, BcryptCompareInfo};
pub use crypto::operations::bcrypt_mod::{Bcrypt, BcryptInfo};
pub use crypto::operations::bcrypt_parse_mod::{BcryptParse, BcryptParseInfo};
pub use crypto::operations::hmac::{HMACInfo, HMAC};
pub use operations::crypto::adler32_checksum_mod::{Adler32CheckSum, Adler32CheckSumInfo};

// endregion

//  region default

pub use operations::default::from_base64_mod::{FromBase64, FromBase64Info};
pub use operations::default::from_base_mod::{FromBase, FromBaseInfo};
pub use operations::default::to_base64_mod::{ToBase64, ToBase64Info};
pub use operations::default::to_base_mod::{ToBase, ToBaseInfo};

// endregion

// region hashing

pub use hashing::operations::analyse_hash_mod::{AnalyseHash, AnalyseHashInfo};
pub use operations::hashing::blake2b_mod::{Blake2b, Blake2bInfo};
pub use operations::hashing::blake2s_mod::{Blake2s, Blake2sInfo};
pub use operations::hashing::md2_mod::{MD2Info, MD2};
pub use operations::hashing::md4_mod::{MD4Info, MD4};
pub use operations::hashing::md5_mod::{MD5Info, MD5};
pub use operations::hashing::sha1_mod::{SHA1Info, SHA1};
pub use operations::hashing::sha2_mod::{SHA2Info, SHA2};
pub use operations::hashing::sha3_mod::{SHA3Info, SHA3};

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
