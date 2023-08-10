use crate::libs::vigenere_trait::VigenereCipher;
use common::{create_operation_struct, error::Error, utils::add, Operation};

create_operation_struct!(VigenereCipherEncode);

impl VigenereCipher for VigenereCipherEncode {}

impl Operation for VigenereCipherEncode {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        VigenereCipherEncode {
            name: "Vigenere Encode",
            module: "Cipher",
            description_en: Some("The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar common based on the letters of a keyword. It is a simple form of polyalphabetic substitution."),
            description_ru: Some("Шифр Виженера — это метод шифрования алфавитного текста с использованием ряда различных общих символов Цезаря, основанных на буквах ключевого слова. Это простая форма полиалфавитной замены."),
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
