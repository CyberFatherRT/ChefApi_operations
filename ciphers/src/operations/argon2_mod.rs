use crate::{
    create_info_struct, create_me_daddy,
    utils::{from_base64, to_hex, DataRepresentation},
    Operation, DOCS_URL,
};
use argon2::{Config, ThreadMode, Variant, Version};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy> for Argon2 {
    fn new(request: String) -> Self {
        Self { request }
    }

    fn run(&self) -> Result<String, String> {
        let request = self.validate(&self.request)?;

        let (params, input) = (request.params, request.input);

        let config = Config {
            variant: params.argon2_type,
            version: Version::Version13,
            mem_cost: params.memory,
            time_cost: params.iterations,
            lanes: params.parallelism,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: params.hash_length,
        };

        let hash = argon2::hash_encoded(input.as_bytes(), params.salt.as_bytes(), &config).unwrap();

        let output = match params.output_format {
            OutputFormat::Encoded => hash,
            format => {
                let raw_hash = hash.split('$').nth(5).unwrap();

                let data = match from_base64(
                    raw_hash.to_string(),
                    "",
                    DataRepresentation::String(String::new()),
                    false,
                    false,
                ) {
                    Ok(DataRepresentation::String(data)) => data,
                    _ => unreachable!(),
                };

                match format {
                    OutputFormat::Hex => to_hex(data.as_bytes()),
                    OutputFormat::Raw => data,
                    _ => unreachable!(),
                }
            }
        };

        Ok(output)
    }
}

// region with structs and enums

#[derive(Deserialize)]
struct Params {
    salt: String,
    iterations: u32,
    memory: u32,
    parallelism: u32,
    hash_length: u32,
    #[serde(with = "MyVariant")]
    argon2_type: Variant,
    output_format: OutputFormat,
}

create_me_daddy!();

/// Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.
/// <br/>
/// For more information go [here](https://wikipedia.org/wiki/Argon2)
/// <br/><br/>
///
/// ### How to use
/// \
/// Send POST requests to /api/Argon2 with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "salt": string,
///         "iterations": u32,
///         "parallelism": u32,
///         "hash_length": u32,
///         "argon2_type": Argon2Type,
///         "output_format": OutputFormat,
///         "memory": u32
///     }
/// }
/// ```
/// #### where
///     - u32 is unsigned 32-bit integer
///     - Argon2Type is one of "Argon2i", "Argon2d", "Argon2id"
///     - OutputFormat is one of "Encoded", "Hex", "Raw"
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
/// POST /api/Argon2
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
/// #### №2
/// ``` http
/// POST /api/Argon2
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
/// #### №3
/// ``` http
/// POST /api/Argon2
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
pub struct Argon2 {
    request: String,
}

#[derive(Deserialize)]
#[serde(remote = "Variant")]
enum MyVariant {
    Argon2d = 0,
    Argon2i = 1,
    Argon2id = 2,
}

#[derive(Deserialize)]
enum OutputFormat {
    Encoded,
    Hex,
    Raw,
}

// endregion

// region info about operation

const NAME: &str = "Argon2";
const DESCRIPTION_EN: &str = "Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.<br><br>Enter the password in the input to generate its hash.";
const DESCRIPTION_RU: &str = "Argon2 – это функция получения ключа, которая была выбрана победителем конкурса хеширования паролей в июле 2015 года. Она была разработана Алексом Бирюковым, Даниэлем Дину и Дмитрием Ховратовичем из Люксембургского университета.<br><br>Введите пароль в ввод для генерации его хэша.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Argon2");

create_info_struct!(
    Argon2Info,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
