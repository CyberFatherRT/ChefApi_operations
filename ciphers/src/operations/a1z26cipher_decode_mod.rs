use crate::{regex_check, utils::char_repr, Operation};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for A1Z26CipherDecode {
    fn new(request: String) -> Self {
        Self { request }
    }

    fn run(&self) -> Result<String, String> {
        let request = self.validate(&self.request)?;

        let delimiter = char_repr(&request.params.delimiter);

        let cipher_text = request
            .input
            .trim_matches(|c: char| [delimiter].contains(&&*c.to_string()))
            .split(delimiter);

        let mut plain_text = String::new();

        for c in cipher_text {
            let c = c.parse::<u32>().unwrap();
            if !(1..=26).contains(&c) {
                return Err("All numbers must be between 1 and 26.".to_string());
            }
            plain_text.push(char::from_u32(c + 96).unwrap());
        }
        Ok(plain_text)
    }

    fn validate(&self, request: &str) -> Result<DeserializeMeDaddy, String> {
        let request = self.deserialize(request)?;

        let regex_checked = match &*request.params.delimiter {
            "Space" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]) ?)+\s*$" => &request.input)
            }
            "Comma" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]),?)+\s*$" => &request.input)
            }
            "Semi-colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]);?)+\s*$" => &request.input)
            }
            "Colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]):?)+\s*$" => &request.input)
            }
            "Line feed" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\n?)+\s*$" => &request.input)
            }
            "CRLF" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\r\n?)+\s*$" => &request.input)
            }
            _ => false,
        };
        if !regex_checked {
            return Err(format!(
                "Invalid delimiter: `{delimiter}`",
                delimiter = request.params.delimiter
            ));
        }

        Ok(request)
    }
}
#[derive(Deserialize)]
struct Params {
    delimiter: String,
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    input: String,
    params: Params,
}

/// A1Z26 is a simple substitution cipher where each letter is replaced by its serial number in the alphabet.
/// <br/><br/>
/// ### How to use
/// \
/// Send POST requests to /api/Argon2 with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "delimiter": string
///     }
/// }
/// ```
/// #### where
///     - delimiter is one of "Space", "Comma", "Semi-colon", "Colon", "Line feed", "CRLF"
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
/// POST /api/A1Z26CipherDecode
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": "8 5 12 12 15",
///     "params": {
///         "delimiter": "Space"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "hello"
/// }
/// ```
/// #### №2
/// ``` http
/// POST /api/A1Z26CipherDecode
/// content_type: application/json; charset=utf-8
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
/// POST /api/A1Z26CipherDecode
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": "4 1 21 15 3",
///     "params": {
///         "delimiter": "Unsupported delimiter"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Invalid delimiter: `Unsupported delimiter`"
/// }
/// ```

pub struct A1Z26CipherDecode {
    request: String,
}

#[derive(Serialize)]
pub struct A1Z26CipherDecodeInfo {
    name: &'static str,
    documentation: &'static str,
    description_en: &'static str,
    description_ru: &'static str,
    info_url: Option<&'static str>,
}

impl A1Z26CipherDecodeInfo {
    pub fn info() -> String {
        let structure = Self {
            name: "A1Z26 Cipher Decode",
            documentation: "soon I transfer all documentation to somewhere :/",
            description_en:
                "Converts alphabet order numbers into their corresponding  alphabet character.",
            description_ru:
                "Преобразует порядковые номера алфавита в соответствующие им символы алфавита.",
            info_url: None,
        };
        serde_json::to_string(&structure).unwrap()
    }
}
