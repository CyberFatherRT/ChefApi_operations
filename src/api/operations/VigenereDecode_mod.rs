use regex::Regex;

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
            request: input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        let alp = match &*self.request.lang {
            "en" => "abcdefghijklmnopqrstuvwxyz",
            "ru_with_yo" => "абвгдеёжзийклмнопрстуфхцчшщъыьэюя",
            "ru" => "абвгдежзийклмнопрстуфхцчшщъыьэюя",
            _ => unreachable!()
        };

        let mut map = std::collections::HashMap::new();
        for (k, v) in alp.chars().enumerate() {
            map.insert(v, k);
        }


        let key = &self.request.params[0].to_lowercase();
        let mut index = 0usize;
        let mut cipher_text = String::new();

        for c in self.request.input.chars() {
            if !c.is_alphabetic() {
                cipher_text.push(c);
                continue;
            }

            let key_idx = map.get(&key.chars().nth(index % key.len()).unwrap())
                .unwrap()
                .to_owned() as i16;

            let text_idx = match c.is_lowercase() {
                true => map.get(&c).unwrap(),
                false => map.get(&c.to_lowercase().next().unwrap()).unwrap(),
            }.to_owned() as i16;

            cipher_text.push(match c.is_lowercase() {
                true => {
                    alp.chars()
                        .nth((text_idx - key_idx).rem_euclid(alp.len() as i16) as usize)
                        .unwrap()
                }
                false => {
                    alp.chars()
                        .nth((text_idx - key_idx).rem_euclid(alp.len() as i16) as usize)
                        .unwrap()
                        .to_uppercase()
                        .next()
                        .unwrap()
                }
            });

            index += 1;
        }

        Ok(cipher_text)
    }

    fn validate<>(&self) -> Result<(), Error> {
        let langs = ["en", "ru", "ru_with_yo"];

        if !langs.contains(&&*self.request.lang) {
            return Err(Error::UnsupportedLanguageError);
        }

        if self.request.params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError);
        }

        let rg = Regex::new(r"^([a-zA-Z]+)|([а-яА-ЯёЁ]+)$").unwrap();

        if !rg.is_match(&self.request.params[0]) {
            return Err(Error::IvalidKeyError);
        }

        Ok(())
    }
}