use super::Operation;

use crate::{error::Error, libs::VigenereCipher, macros::create_struct, utils::add};

create_struct!(VigenereCipherEncode);

impl VigenereCipher for VigenereCipherEncode {}

impl Operation for VigenereCipherEncode {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        VigenereCipherEncode {
            name: "Vigenere Encode",
            module: "Cipher",
            description: Some("The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar ciphers based on the letters of a keyword. It is a simple form of polyalphabetic substitution."),
            info_url: Some("https://wikipedia.org/wiki/Vigenère_cipher"),
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        <Self as VigenereCipher>::cipher(&self.lang, &self.params, &self.input, add)
    }

    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
