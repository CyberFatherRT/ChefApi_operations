use crate::{
    create_info_struct, create_me_daddy, libs::ciphers::affine_cipher_encode,
    utils::SupportedLanguage, Operation, DOCS_URL,
};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, String> for AtbashCipherEncode {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;

        let (input, lang) = (request.input, request.params.lang);

        match lang {
            SupportedLanguage::En => affine_cipher_encode(&input, lang, 25, 25),
            SupportedLanguage::Ru => affine_cipher_encode(&input, lang, 31, 31),
            SupportedLanguage::RuWithYo => affine_cipher_encode(&input, lang, 32, 32),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguage,
}

create_me_daddy!();

/// Atbash is a mono-alphabetic substitution cipher originally used to encode the Hebrew alphabet. It has been modified here for use with the Latin alphabet and Cyrillic.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Atbash).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/AtbashCipherEncode with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "lang": SupportedLanguage,
///     }
/// }
/// ```
/// #### where
///     - SupportedLanguages is enum of "en", "ru", "ru_with_yo".
/// <br/><br/>
///
/// ### Server response have two possible formats
///
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
/// ``` json
/// { "Ok": `some answer` }
/// ```
/// #### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
/// ``` json
/// { "Err": `error message` }
/// ```
/// # Examples
/// ## №1
/// ``` http
/// POST /api/AtbashCipherEncode
///
/// {
///     "input": "hello",
///     "params": {
///         "lang": "en"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "svool"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/AtbashCipherEncode
///
/// {
///     "input": "Привет!",
///     "params": {
///         "lang": "ru",
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Рпчэън!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/AtbashCipherEncode
///
/// {
///     "input": "no lang?",
///     "params": {
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "missing field `lang`"
/// }
/// ```
pub struct AtbashCipherEncode;

const NAME: &str = "AtbashCipher";
const DESCRIPTION_EN: &str = "Atbash is a mono-alphabetic substitution cipher originally used to encode the Hebrew alphabet. It has been modified here for use with the Latin alphabet and Cyrillic.";
const DESCRIPTION_RU: &str = "Атбаш — это моноалфавитный шифр замены, изначально использовавшийся для кодирования еврейского алфавита. Здесь он был изменен для использования с латинским алфавитом и кириллицей";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Atbash");

create_info_struct!(
    AtbashCipherInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
