use super::{Operation, Request};

use crate::api::{
    macros::create_struct,
    utils::add,
    error::Error,
    lib::VigenereCipher,
};

create_struct!(VigenereEncode);

impl VigenereCipher for VigenereEncode {}

impl Operation for VigenereEncode {
    fn new(input: Request) -> Box<Self> {
        Box::new(VigenereEncode {
            name: "Vigenere Encode",
            module: "Cipher",
            description: Some("The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar ciphers based on the letters of a keyword. It is a simple form of polyalphabetic substitution."),
            infoURL: Some("https://wikipedia.org/wiki/Vigenère_cipher"),
            request: input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        <Self as VigenereCipher>::cipher(&self.request, add)
    }

    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

