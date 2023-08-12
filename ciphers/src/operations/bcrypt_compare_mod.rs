use crate::{create_info_struct, create_me_daddy, Operation, DOCS_URL};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, String> for BcryptCompare {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, encoded_hash) = (request.input, request.params.encoded_hash);

        let res = bcrypt::verify(input.as_bytes(), &encoded_hash).map_err(|err| err.to_string())?;

        match res {
            true => Ok(format!("Match `{}`.", input)),
            false => Err("No match.".to_string()),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    encoded_hash: String,
}

create_me_daddy!();

/// Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Bcrypt)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/BcryptCompare with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "encoded_hash": String
///     }
/// }
/// ```
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
/// POST /api/BcryptCompare
///
/// {
///     "input": "hello",
///     "params": {
///         "encoded_hash": "$2b$12$mLDUe/nTaPt06W2ai4YrVeCiPK7/L1Dhj7FipakSCnKIDsgqbvPgm"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "Match `hello`."
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/BcryptCompare
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "encoded_hash": "$2x$04$hC6BHE9hPEQZExczLDTxBOgq48yNMI7HC5bmE0HiP/iGxtMpwryh6"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "Match `Привет, Мир!`."
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/BcryptCompare
///
/// {
///     "input": "missing encoded_hash",
///     "params": {
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "Missing field `encoded_hash`."
/// }
/// ```

pub struct BcryptCompare;

const NAME: &str = "BcryptCompare";
const DESCRIPTION_EN: &str = "bcrypt is a password hashing function designed by Niels Provos and David Mazières, based on the Blowfish cipher, and presented at USENIX in 1999. Besides incorporating a salt to protect against rainbow table attacks, bcrypt is an adaptive function: over time, the iteration count (rounds) can be increased to make it slower, so it remains resistant to brute-force search attacks even with increasing computation power.";
const DESCRIPTION_RU: &str = "bcrypt — это функция хеширования паролей, разработанная Нильсом Провосом и Давидом Мазьером на основе шифра Blowfish и представленная на USENIX в 1999 году. Помимо включения соли для защиты от RainbowTableAttack, bcrypt является адаптивной функцией: со временем количество итераций (раундов) может быть увеличено, чтобы сделать его медленнее, поэтому он остается устойчивым к поисковым атакам методом грубой силы даже при увеличении вычислительной мощности.";

const INFO_URL: Option<&str> = Some("https://wikipedia.org/wiki/Bcrypt");

create_info_struct!(
    BcryptCompareInfo,
    NAME,
    DOCS_URL,
    DESCRIPTION_EN,
    DESCRIPTION_RU,
    INFO_URL
);
