pub mod error;
mod macros;
pub mod traits;
pub mod utils;

use error::Error;
use serde::{de, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Operations {
    A1Z26CipherDecode,
    A1Z26CipherEncode,
    AffineCipherDecode,
    AffineCipherEncode,
    Argon2,
    AnalyseHash,
    FromBase64,
    ToBase64,
    VigenereCipherDecode,
    VigenereCipherEncode,
}

pub trait Operation {
    fn new(lang: String, params: Vec<String>, input: String) -> Self;
    fn run(&self) -> Result<String, Error>;
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub fn deserialize_json<'a, T>(json: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let temp: T = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(err) => {
            return Err(Error::Error {
                error: err.to_string(),
            })
        }
    };

    Ok(temp)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub output: Result<String, Error>,
}

impl Response {
    pub fn new(output: Result<String, Error>) -> Self {
        Self { output }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub name: Operations,
    pub lang: String,
    pub params: Vec<String>,
    pub input: String,
}

impl Request {
    pub fn new(name: Operations, lang: String, params: Vec<String>, input: String) -> Self {
        Self {
            name,
            lang,
            params,
            input,
        }
    }
}
