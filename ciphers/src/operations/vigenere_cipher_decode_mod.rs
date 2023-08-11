use crate::{
    create_me_daddy,
    libs::vigenere_trait::VigenereCipher,
    utils::{sub, SupportedLanguage},
    Operation,
};
use serde::Deserialize;

impl VigenereCipher for VigenereCipherDecode {}

impl Operation<'_, DeserializeMeDaddy, String> for VigenereCipherDecode {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, lang, key) = (request.input, request.params.lang, request.params.key);
        <Self as VigenereCipher>::cipher(lang, &key, &input, sub)
    }
}

/// The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar common based on the letters of a keyword. It is a simple form of polyalphabetic substitution.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Base64).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/VigenereCipherDecode with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "lang": SupportedLanguages,
///         "key": String
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
/// POST /api/VigenereCipherDecode
///
/// {
///     "input": "Rijvs, Uyvjn!",
///     "params": {
///         "lang": "en",
///         "key": "key"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Hello, World!"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/VigenereCipherDecode
///
/// {
///     "input": "Ееклыз, Осж!",
///     "params": {
///         "lang": "ru",
///         "key": "ключ"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Привет, Мир!"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/VigenereCipherDecode
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": "hrbtr lntrunmp",
///     "params": {
///         "lang": "else",
///         "key": "lang"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Unknown variant `else`, expected one of `En`, `Ru`, `RuAlpWithYo`."
/// }
/// ```
pub struct VigenereCipherDecode;

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguage,
    key: String,
}

create_me_daddy!();
