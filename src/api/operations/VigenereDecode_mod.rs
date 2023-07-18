use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::api::error::Error;
use super::{Operation, Request};
use crate::api::macros::create_struct;

create_struct!(VigenereDecode);

impl Operation for VigenereDecode {
    fn new(input: Request) -> Box<Self> {
        Box::new(VigenereDecode {
            name: "Vigenere Decode",
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

        let (alp, reg) = match &*self.request.lang {
            "en" => ("abcdefghijklmnopqrstuvwxyz", r"^[a-zA-Z]+$"),
            "ru" => ("абвгдежзийклмнопрстуфхцчшщъыьэюя", "^[а-яА-Я]+$"),
            "ru_with_yo" => ("абвгдеёжзийклмнопрстуфхцчшщъыьэюя", r"^[а-яА-ЯёЁ]+$"),
            _ => unreachable!()
        };

        let mut map = std::collections::HashMap::new();
        for (k, v) in alp.chars().enumerate() {
            map.insert(v, k);
        }


        let key = &self.request.params[0].to_lowercase();
        let rg = Regex::new(reg).unwrap();
        let mut index = 0usize;
        let mut cipher_text = String::new();

        let key_len = key.graphemes(true).count();
        let alp_len = alp.graphemes(true).count() as i16;



        for c in self.request.input.chars() {
            if !rg.is_match(&c.to_string()) {
                cipher_text.push(c);
                continue;
            }

            let key_idx = map.get(&key.chars().nth(index % key_len).unwrap())
                .unwrap()
                .to_owned() as i16;

            let text_idx = match c.is_lowercase() {
                true => map.get(&c).unwrap(),
                false => map.get(&c.to_lowercase().next().unwrap()).unwrap(),
            }.to_owned() as i16;


            cipher_text.push(match c.is_lowercase() {
                true => {
                    alp.chars()
                        .nth((text_idx - key_idx).rem_euclid(alp_len) as usize)
                        .unwrap()
                }
                false => {
                    alp.chars()
                        .nth((text_idx - key_idx).rem_euclid(alp_len) as usize)
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

        let lang_reg = match self.request.lang.as_str() {
            "en" => Regex::new(r"^[a-zA-Z]+$").unwrap(),
            "ru" => Regex::new(r"^[а-яА-Я]+$").unwrap(),
            "ru_with_yo" => Regex::new(r"^[а-яА-ЯёЁ]+$").unwrap(),
            _ => return Err(Error::UnsupportedLanguageError),
        };

        if !lang_reg.is_match(&self.request.params[0]) {
            return Err(Error::IvalidKeyError);
        }

        Ok(())
    }
}