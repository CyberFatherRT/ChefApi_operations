use super::{Operation, Request};

use crate::api::{error::Error, lib::VigenereCipher, macros::create_struct, utils::sub};

create_struct!(VigenereCipherDecode);

impl VigenereCipher for VigenereCipherDecode {}

impl Operation for VigenereCipherDecode {
    fn new(input: Request) -> Self {
        VigenereCipherDecode {
            name: "Vigenere Decode",
            module: "Cipher",
            description: Some("The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar ciphers based on the letters of a keyword. It is a simple form of polyalphabetic substitution."),
            infoURL: Some("https://wikipedia.org/wiki/Vigenère_cipher"),
            request: input,
        }
    }

    fn run(&self) -> Result<String, Error> {
        <Self as VigenereCipher>::cipher(&self.request, sub)
    }

    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
