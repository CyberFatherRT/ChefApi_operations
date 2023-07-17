pub mod A1Z26CipherDecode;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Operation {
    pub name: String,
    pub lang: String,
    pub params: Vec<String>,
    pub input: String,
}
