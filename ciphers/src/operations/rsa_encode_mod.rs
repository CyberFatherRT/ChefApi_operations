use crate::utils::byte_array_to_string;
use crate::{create_me_daddy, Operation};
use rsa::{
    pkcs1::DecodeRsaPublicKey,
    sha2::{Sha256, Sha384, Sha512},
    traits::PaddingScheme,
    Oaep, Pkcs1v15Encrypt, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use sha1::Sha1;

pub struct RSAEncrypt;

impl Operation<'_, DeserializeMeDaddy, String> for RSAEncrypt {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, public_key, encrypted_scheme, message_digest_algorithm) = (
            request.input,
            request.params.public_key,
            request.params.encrypted_scheme,
            request.params.message_digest_algorithm,
        );

        if matches!(encrypted_scheme, SupportedEncryptionSchemes::RSA_OAEP)
            && message_digest_algorithm.is_none()
        {
            return Err("RSA_OAEP must have message digest algorithm".to_string());
        }

        let pubkey = RsaPublicKey::from_pkcs1_pem(&public_key).map_err(|err| err.to_string())?;
        let mut rng = rand::thread_rng();
        let encrypted_text = match encrypted_scheme {
            SupportedEncryptionSchemes::RSA_OAEP => {
                let padding = match message_digest_algorithm.unwrap() {
                    SupportedMessageDigestAlgorithm::SHA_1 => Oaep::new::<Sha1>(),
                    SupportedMessageDigestAlgorithm::SHA_256 => Oaep::new::<Sha256>(),
                    SupportedMessageDigestAlgorithm::SHA_384 => Oaep::new::<Sha384>(),
                    SupportedMessageDigestAlgorithm::SHA_512 => Oaep::new::<Sha512>(),
                };
                pubkey.encrypt(&mut rng, padding, input.as_bytes())
            }
            SupportedEncryptionSchemes::RSA_AES_PKCS1_V1_5 => {
                Pkcs1v15Encrypt::encrypt(Pkcs1v15Encrypt, &mut rng, &pubkey, input.as_bytes())
            }
        }
        .map_err(|err| err.to_string())?;

        byte_array_to_string(encrypted_text)
    }
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum SupportedEncryptionSchemes {
    #[serde(rename = "rsa_oaep")]
    RSA_OAEP,
    #[serde(rename = "rsa_aes_pkcs1_v1_5")]
    RSA_AES_PKCS1_V1_5,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum SupportedMessageDigestAlgorithm {
    #[serde(rename = "sha1")]
    SHA_1,
    #[serde(rename = "sha256")]
    SHA_256,
    #[serde(rename = "sha384")]
    SHA_384,
    #[serde(rename = "sha512")]
    SHA_512,
}

#[derive(Deserialize)]
struct Params {
    public_key: String,
    encrypted_scheme: SupportedEncryptionSchemes,
    message_digest_algorithm: Option<SupportedMessageDigestAlgorithm>,
}

create_me_daddy!();
