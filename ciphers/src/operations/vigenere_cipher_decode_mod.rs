use crate::{
    create_me_daddy,
    libs::vigenere_trait::VigenereCipher,
    utils::{sub, SupportedLanguage},
    Operation,
};
use serde::Deserialize;

impl VigenereCipher for VigenereCipherDecode {}

#[derive(Deserialize)]
struct Params {
    lang: SupportedLanguage,
    key: String,
}

create_me_daddy!();

pub struct VigenereCipherDecode;

impl Operation<'_, DeserializeMeDaddy, String> for VigenereCipherDecode {
    fn run(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (input, lang, key) = (request.input, request.params.lang, request.params.key);
        <Self as VigenereCipher>::cipher(lang, &key, &input, sub)
    }
}
