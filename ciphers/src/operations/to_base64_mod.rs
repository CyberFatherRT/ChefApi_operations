use crate::{create_me_daddy, utils::to_base64, Operation};
use serde::Deserialize;

impl Operation<'_, DeserializeMeDaddy, String> for ToBase64 {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, alphabet) = (request.input, request.params.alphabet);
        to_base64(&input, alphabet)
    }
}

#[derive(Deserialize)]
struct Params {
    alphabet: Option<String>,
}

create_me_daddy!();

/// Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Base64).
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/ToBase64 with your data using json payload with this structure.
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "alphabet": Option<string>,
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
/// POST /api/ToBase64
///
/// {
///     "input": "hello",
///     "params": {
///         "alphabet": "",
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
/// POST /api/ToBase64
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
/// POST /api/ToBase64
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

pub struct ToBase64;
