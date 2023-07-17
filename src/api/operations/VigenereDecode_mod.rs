use serde_json::to_string;
use crate::api::error::Error;
use super::{Operation, Request};
use crate::api::macros::create_struct;

create_struct!(VigenereDecode);

impl Operation for VigenereDecode {
    fn new(input: Request) -> Box<Self> {
        Box::new(VigenereDecode {
            name: "VigenereDecode",
            module: "Cipher",
            description: Some("The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar ciphers based on the letters of a keyword. It is a simple form of polyalphabetic substitution."),
            infoURL: Some("https://wikipedia.org/wiki/Vigenère_cipher"),
            input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        let alp = match &*self.input.lang {
            "en" => "abcdefghijklmnopqrstuvwxyz",
            "ru" => "абвгдеёжзийклмнопрстуфхцчшщъыьэюя",
            _ => "abcdefghijklmnopqrstuvwxyz"
        };

        Ok(String::new())
    }

    fn validate(&self) -> Result<(), Error> {
        let langs = ["en", "ru"];

        if !langs.contains(&&*self.input.lang) {
            return Err(Error::UnsupportedLanguage(lang: self.input.lang));
        }

        Ok(())
    }
}