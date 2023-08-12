use crate::{create_me_daddy, Operation};
use bcrypt::Version;
use serde::Deserialize;

// TODO: Are you remember about ME????
fn delete_me_after_you_make_pay_system() -> u32 {
    12
}

impl Operation<'_, DeserializeMeDaddy, String> for Bcrypt {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, rounds, version) =
            (request.input, request.params.rounds, request.params.version);

        let res =
            bcrypt::hash_with_result(input.as_bytes(), rounds).map_err(|err| err.to_string())?;
        Ok(res.format_for_version(version).to_string())
    }

    fn validate(&self, request: &'_ str) -> Result<DeserializeMeDaddy, String> {
        let request = self.deserialize(request)?;
        if request.params.rounds > delete_me_after_you_make_pay_system() {
            return Err("Rounds must be between 4 and 12".to_string());
        }
        Ok(request)
    }
}

#[derive(Deserialize)]
#[serde(remote = "Version")]
pub enum MyVersion {
    #[serde(alias = "2a")]
    TwoA,
    #[serde(alias = "2x")]
    TwoX,
    #[serde(alias = "2y")]
    TwoY,
    #[serde(alias = "2b")]
    TwoB,
}

#[derive(Deserialize)]
struct Params {
    rounds: u32,
    #[serde(with = "MyVersion")]
    version: Version,
}

create_me_daddy!();

/// Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.
/// <br><br/>
/// For more information go [here](https://wikipedia.org/wiki/Bcrypt)
/// <br><br/>
///
/// # How to use
/// \
/// Send POST requests to /api/Bcrypt with your data using json payload with this structure
/// ``` json
/// {
///     "input": string,
///     "params": {
///         "rounds": u31,
///         "version": Version
///     }
/// }
/// ```
/// #### where
///     - u31 is unsigned digit between 4 and 31
///     - Version is enum of "2a", "2x", "2y", "2b"
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
/// POST /api/Bcrypt
///
/// {
///     "input": "hello",
///     "params": {
///         "rounds": 12,
///         "version": "2b"
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 200 Ok
/// {
///   "Ok": "$2b$12$mLDUe/nTaPt06W2ai4YrVeCiPK7/L1Dhj7FipakSCnKIDsgqbvPgm"
/// }
/// ```
/// ## №2
/// ``` http
/// POST /api/Bcrypt
///
/// {
///     "input": "Привет, Мир!",
///     "params": {
///         "rounds": 4,
///         "version": "2x"
///     }
/// }
/// ```
/// ```http
/// {
///   "Ok": "$2x$04$hC6BHE9hPEQZExczLDTxBOgq48yNMI7HC5bmE0HiP/iGxtMpwryh6"
/// }
/// ```
/// ## №3
/// ``` http
/// POST /api/Bcrypt
///
/// {
///     "input": "missing version",
///     "params": {
///         "rounds": 4
///     }
/// }
/// ```
/// ```http
/// HTTP/1.1 400 Bad Request
/// {
///   "Err": "missing field `version`"
/// }
/// ```
pub struct Bcrypt;
