use super::{Operation, Request};

use crate::api::{
    macros::create_struct,
    utils::sub,
    error::Error,
    lib::VigenereCipher,
};

create_struct!(VigenereDecode);

impl VigenereCipher for VigenereDecode {}

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
        <Self as VigenereCipher>::cipher(&self.request, sub)
    }

    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

