use crate::utils::modulus;
use crate::{
    create_info_struct, lang_me_daddy,
    traits::CharTrait,
    utils::{get_alphabet, get_char_by_index, get_index_by_char, mod_inv, validate_lang},
    Operation, DOCS_URL,
};
use num::Integer;
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for AffineCipherDecode {
    fn new(request: String) -> Self {
        Self { request }
    }

    fn run(&self) -> Result<String, String> {
        let request = self.validate(&self.request)?;

        let (input, lang, params) = (request.input, request.lang, request.params);
        if !validate_lang(&input, &lang) {
            return Err("Wrong language.".to_string());
        };

        let (a, b) = (params.a as i16, params.b as i16);

        let (alp_lower, alp_upper, alp_length, _) = get_alphabet(&lang);
        if a.gcd(&(alp_length as i16)) != 1 {
            return Err(format!(
                "The value of `a` must be coprime to alphabet length({}).",
                alp_length
            ));
        }

        let mut output = String::with_capacity(alp_length as usize);

        for c in input.chars() {
            if !c.is_alphabetic() {
                output.push(c);
                continue;
            }

            let y = match c.is_lowercase() {
                true => get_index_by_char(alp_lower, c),
                false => get_index_by_char(alp_upper, c),
            } as i16;

            let inv_a = mod_inv(a, alp_length as i16);

            let x = modulus(inv_a * (y - b), alp_length as i16);

            output.push(match c.is_lowercase() {
                true => get_char_by_index(alp_lower, x),
                false => get_char_by_index(alp_upper, x).to_upper_case(),
            });
        }

        Ok(output)
    }
}

#[derive(Deserialize)]
struct Params {
    a: u8,
    b: u8,
}

lang_me_daddy!();

/// The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Argon2)
/// <br><br/>
/// ### How to use
/// \
/// Send POST requests to /api/AffineCipherDecode with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "a": u8,
///         "b": u8,
///     }
/// }
/// ```
/// #### where
///     - u8 is unsigned 8-bit integer (digit between 0 and 255)
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
/// ### Examples
/// <br><br/>
/// #### №1
/// ``` http
/// POST /api/AffineCipherDecode
///
/// {
///     "input": "Cnwwl, Zlawi!",
///     "lang": "en",
///     "params": {
///         "a": 5,
///         "b": 19
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Hello, World!"
/// }
/// ```
/// #### №2
/// ``` http
/// POST /api/AffineCipherDecode
///
/// {
///     "input": "18;9;3;11;18;15;12;12",
///     "params": {
///         "delimiter": "Semi-colon"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "rickroll"
/// }
/// ```
/// #### №3
/// ``` http
/// POST /api/AffineCipherDecode
///
/// {
///     "input": "Cnwwl, Zlawi!",
///     "lang": "en",
///     "params": {
///         "a": -5,
///         "b": 12735073052703957225979
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Invalid delimiter: `Unsupported delimiter`"
/// }
/// ```
pub struct AffineCipherDecode {
    request: String,
}

const NAME: &str = "AffineCipherDecode";
const DESCRIPTION_EN: &str = "The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter.";
const DESCRIPTION_RU: &str = "Аффинный шифр — это тип моноалфавитного шифра замены. Чтобы расшифровать, каждая буква в алфавите сопоставляется с ее числовым эквивалентом, расшифровывается с помощью математической функции и преобразуется обратно в букву.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Affine_cipher");

create_info_struct!(
    AffineCipherDecodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
