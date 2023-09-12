use serde::{Deserialize, Serialize};

use utils::{
    create_info_struct, create_me_daddy, regex_check, utils::char_repr, Operation, DOCS_URL,
};
impl Operation<'_, DeserializeMeDaddy, String> for A1Z26CipherDecode {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;

        let delimiter = char_repr(&request.params.delimiter);

        let cipher_text = request.input.split(delimiter);

        let mut plain_text = String::new();

        for c in cipher_text {
            let Ok(c) = c.parse::<u8>() else { continue };
            if !(1..=26).contains(&c) {
                return Err("All numbers must be between 1 and 26.".to_string());
            }
            plain_text.push((c + 96) as char);
        }
        Ok(plain_text)
    }

    fn validate(&self, request: &str) -> Result<DeserializeMeDaddy, String> {
        let request = self.deserialize(request)?;

        let regex_checked = match &*request.params.delimiter {
            "Space" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]) *)+\s*$" => &request.input)
            }
            "Comma" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]),*)+\s*$" => &request.input)
            }
            "Semi-colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]);*)+\s*$" => &request.input)
            }
            "Colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]):*)+\s*$" => &request.input)
            }
            "Line feed" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\n*)+\s*$" => &request.input)
            }
            "CRLF" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\r\n*)+\s*$" => &request.input)
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

create_me_daddy!();

/// A1Z26 is a simple substitution cipher where each letter is replaced by its serial number in the alphabet.
/// <br/><br/>
/// # How to use
/// \
/// Send POST requests to /api/A1Z26CipherDecode with your data using json payload with this structure
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
/// # Examples
/// <br><br/>
/// ## №1
/// ``` http
/// POST /api/A1Z26CipherDecode
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
/// ## №2
/// ``` http
/// POST /api/A1Z26CipherDecode
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
/// ## №3
/// ``` http
/// POST /api/A1Z26CipherDecode
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

pub struct A1Z26CipherDecode;

const NAME: &str = "A1Z26CipherDecode";
const DESCRIPTION_EN: &str =
    "Converts alphabet order numbers into their corresponding alphabet character.";
const DESCRIPTION_RU: &str =
    "Преобразует порядковые номера алфавита в соответствующие им символы алфавита.";

const INFO_URL: Option<&str> = None;

create_info_struct!(
    A1Z26CipherDecodeInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
