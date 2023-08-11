use crate::{
    create_info_struct, create_me_daddy,
    utils::{from_base64, DataRepresentation},
    Operation, DOCS_URL,
};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, String> for FromBase64 {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;

        let (input, alphabet, remove_non_alphabetic_chars, strict_mode) = (
            request.input,
            request.params.alphabet,
            request.params.remove_non_alphabetic_chars,
            request.params.strict_mode,
        );

        let alphabet = alphabet.unwrap_or(String::from(""));

        match from_base64(
            input,
            &alphabet,
            DataRepresentation::String(String::new()),
            remove_non_alphabetic_chars,
            strict_mode,
        ) {
            Ok(output) => {
                let DataRepresentation::String(output) = output else {
                    unreachable!()
                };
                Ok(output.trim_end_matches('\0').to_string())
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    alphabet: Option<String>,
    remove_non_alphabetic_chars: bool,
    strict_mode: bool,
}

create_me_daddy!();

/// Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Base64).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Base64 with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "alphabet": Option<string>,
///         "remove_non_alphabetic_chars": bool,
///         "strict_mode": bool
///     }
/// }
/// ```
/// #### where
///     - Option<string> is type that can be string or null.
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
/// POST /api/Base64
///
/// {
///     "input": "hello",
///     "params": {
///         "salt": "somesalt",
///         "iterations": 3,
///         "parallelism": 1,
///         "hash_length": 32,
///         "argon2_type": "Argon2i",
///         "output_format": "Encoded",
///         "memory": 4096
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$WVDOfucSPAey3UEzzqLtBwRbGS83pTyIPLXgjhKfgrY"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Base64
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "salt": "новая соль",
///         "iterations": 6,
///         "parallelism": 1,
///         "hash_length": 34,
///         "argon2_type": "Argon2id",
///         "output_format": "Hex",
///         "memory": 8096
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "eb4140b78ed1c4fcd736c1b73cdf555ba244371ff53971e53823e411aeefbd60751d"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Base64
/// content_type: application/json; charset=utf-8
///
/// {
///     "input": "error",
///     "params": {
///         "salt": "missing iterations parameter",
///         "parallelism": 1,
///         "hash_length": 34,
///         "argon2_type": "Argon2id",
///         "output_format": "Hex",
///         "memory": 8096
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "missing field `iterations`"
/// }
/// ```
pub struct FromBase64;

const NAME: &str = "FromBase64";
const DESCRIPTION_EN: &str = "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.";
const DESCRIPTION_RU: &str = "Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция декодирует необработанные данные в строку ASCII Base64.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Base64");

create_info_struct!(
    FromBase64Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);

struct _AlphabetOptions {
    name: &'static str,
    value: &'static str,
}

const _ALPHABET_OPTIONS: &[_AlphabetOptions] = &[
    _AlphabetOptions {
        name: "Standard (RFC 4648): A-Za-z0-9+/=",
        value: "A-Za-z0-9+/=",
    },
    _AlphabetOptions {
        name: "URL safe (RFC 4648 §5): A-Za-z0-9-_",
        value: "A-Za-z0-9-_",
    },
    _AlphabetOptions {
        name: "Filename safe: A-Za-z0-9+-=",
        value: "A-Za-z0-9+\\-=",
    },
    _AlphabetOptions {
        name: "itoa64: ./0-9A-Za-z=",
        value: "./0-9A-Za-z=",
    },
    _AlphabetOptions {
        name: "y64: A-Za-z0-9._-",
        value: "A-Za-z0-9._-",
    },
    _AlphabetOptions {
        name: "z64: 0-9a-zA-Z+/=",
        value: "0-9a-zA-Z+/=",
    },
    _AlphabetOptions {
        name: "Radix-64 (RFC 4880): 0-9A-Za-z+/=",
        value: "0-9A-Za-z+/=",
    },
    _AlphabetOptions {
        name: "Uuencoding: [space]-_",
        value: " -_",
    },
    _AlphabetOptions {
        name: "Xxencoding: +-0-9A-Za-z",
        value: "+\\-0-9A-Za-z",
    },
    _AlphabetOptions {
        name: "BinHex: !-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
        value: "!-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
    },
    _AlphabetOptions {
        name: "ROT13: N-ZA-Mn-za-m0-9+/=",
        value: "N-ZA-Mn-za-m0-9+/=",
    },
    _AlphabetOptions {
        name: "UNIX crypt: ./0-9A-Za-z",
        value: "./0-9A-Za-z",
    },
    _AlphabetOptions {
        name: "Atom128: /128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
        value: "/128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
    },
    _AlphabetOptions {
        name: "Megan35: 3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
        value: "3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
    },
    _AlphabetOptions {
        name: "Zong22: ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
        value: "ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
    },
    _AlphabetOptions {
        name: "Hazz15: HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
        value: "HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
    },
];
