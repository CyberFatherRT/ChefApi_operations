use crate::{
    create_info_struct, create_me_daddy,
    libs::base64::{from_base64, to_base64},
    utils::{to_hex, DataRepresentation},
    Operation, DOCS_URL,
};
use rsa::{pkcs1::DecodeRsaPrivateKey, Oaep, Pkcs1v15Encrypt, RsaPrivateKey};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

impl Operation<'_, DeserializeMeDaddy, OutputFormat> for RSADecrypt {
    fn run(&self, request: &str) -> Result<OutputFormat, String> {
        let request = self.validate(request)?;
        let (input, pem_key, encrypted_scheme, message_digest_algorithm, output_format) = (
            request.input,
            request.params.private_key,
            request.params.encrypted_scheme,
            request.params.message_digest_algorithm,
            request.params.output_format,
        );

        if matches!(encrypted_scheme, SupportedEncryptionSchemes::RSA_OAEP)
            && message_digest_algorithm.is_none()
        {
            return Err("RSA_OAEP must have message digest algorithm".to_string());
        }

        let DataRepresentation::ByteArray(input) = from_base64(
            input,
            "",
            DataRepresentation::ByteArray(Vec::new()),
            false,
            false,
        )?
        else {
            unreachable!()
        };

        let input = &input[..input.len() - 1];

        let pem_key: RsaPrivateKey =
            DecodeRsaPrivateKey::from_pkcs1_pem(&pem_key).map_err(|err| err.to_string())?;

        let encrypted_text = match encrypted_scheme {
            SupportedEncryptionSchemes::RSA_OAEP => {
                let padding = match message_digest_algorithm.unwrap() {
                    SupportedMessageDigestAlgorithm::SHA1 => Oaep::new::<Sha1>(),
                    SupportedMessageDigestAlgorithm::SHA2_224 => Oaep::new::<Sha224>(),
                    SupportedMessageDigestAlgorithm::SHA2_256 => Oaep::new::<Sha256>(),
                    SupportedMessageDigestAlgorithm::SHA2_384 => Oaep::new::<Sha384>(),
                    SupportedMessageDigestAlgorithm::SHA2_512 => Oaep::new::<Sha512>(),
                    SupportedMessageDigestAlgorithm::SHA3_224 => Oaep::new::<Sha3_224>(),
                    SupportedMessageDigestAlgorithm::SHA3_256 => Oaep::new::<Sha3_256>(),
                    SupportedMessageDigestAlgorithm::SHA3_384 => Oaep::new::<Sha3_384>(),
                    SupportedMessageDigestAlgorithm::SHA3_512 => Oaep::new::<Sha3_512>(),
                };
                pem_key.decrypt(padding, input)
            }
            SupportedEncryptionSchemes::RSA_AES_PKCS1_V1_5 => {
                pem_key.decrypt(Pkcs1v15Encrypt, input)
            }
        }
        .map_err(|err| err.to_string())?;

        Ok(match output_format {
            SupportedOutputFormat::Hex => OutputFormat::Hex(to_hex(&encrypted_text)),
            SupportedOutputFormat::Base64 => {
                OutputFormat::Base64(to_base64(&encrypted_text, None).unwrap())
            }
            SupportedOutputFormat::Uint8Array => OutputFormat::Uint8Array(encrypted_text),
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum SupportedEncryptionSchemes {
    #[serde(rename = "oaep")]
    RSA_OAEP,
    #[serde(rename = "pkcs1_v15")]
    RSA_AES_PKCS1_V1_5,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedMessageDigestAlgorithm {
    SHA1,
    SHA2_224,
    SHA2_256,
    SHA2_384,
    SHA2_512,
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SupportedOutputFormat {
    Hex,
    Base64,
    Uint8Array,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Hex(String),
    Base64(String),
    Uint8Array(Vec<u8>),
}

#[derive(Deserialize)]
struct Params {
    #[serde(rename = "pem_key")]
    private_key: String,
    #[serde(rename = "scheme")]
    encrypted_scheme: SupportedEncryptionSchemes,
    #[serde(rename = "digest_alg")]
    message_digest_algorithm: Option<SupportedMessageDigestAlgorithm>,
    output_format: SupportedOutputFormat,
}

create_me_daddy!();

pub struct RSADecrypt;

const NAME: &str = "RSADecrypt";
const DESCRIPTION_EN: &str = "Decrypt a message with a PEM encoded RSA private key.";
const DESCRIPTION_RU: &str = "Дешифрует сообщение с помощью приватного ключа RSA с кодировкой PEM.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/RSA_(cryptosystem)");

create_info_struct!(
    RSADecryptInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
