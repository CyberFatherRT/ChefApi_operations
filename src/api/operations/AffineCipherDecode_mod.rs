use super::{Operation, Request};
use crate::api::{error::Error, lib::AffineCipher, macros::create_struct, utils::getCharByIndex};
use num::Integer;
use unicode_segmentation::UnicodeSegmentation;

create_struct!(AffineCipherDecode);

impl AffineCipher for AffineCipherDecode {}

impl Operation for AffineCipherDecode {
    fn new(input: Request) -> Self {
        AffineCipherDecode {
            name: "Affine Cipher Decode",
            module: "Cipher",
            description: Some("The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter."),
            infoURL: Some("https://wikipedia.org/wiki/Affine_cipher"),
            request: input,
        }
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        let (a, b) = <Self as AffineCipher>::get_a_b(&self.request);
        let (mut plaintext, alp) = <Self as AffineCipher>::get_plaintext_alp(&self.request);

        for c in self.request.input.chars() {
            if !c.is_alphabetic() {
                plaintext.push(c);
                continue;
            }

            plaintext.push(match c.is_lowercase() {
                true => getCharByIndex(alp.0, <Self as AffineCipher>::decode(a, b, c, alp.0)),
                false => getCharByIndex(
                    alp.0,
                    <Self as AffineCipher>::decode(a, b, c.to_lowercase().next().unwrap(), alp.0),
                )
                .to_uppercase()
                .next()
                .unwrap(),
            });
        }

        Ok(plaintext)
    }

    fn validate(&self) -> Result<(), Error> {
        if let Err(e) = <Self as AffineCipher>::validate(&self.request) {
            return Err(e);
        };

        if self
            .request
            .params
            .get(0)
            .unwrap()
            .parse::<isize>()
            .unwrap()
            .gcd(&(self.request.lang.graphemes(true).count() as isize))
            != 1
        {
            return Err(Error::InvalidParamTypeError {
                error: "The value of `a` must be coprime to alphabet length.",
            });
        }

        Ok(())
    }
}
