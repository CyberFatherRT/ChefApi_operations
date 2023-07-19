use super::{Operation, Request};
use crate::api::utils::get_by_index;
use crate::api::{error::Error, lib::AffineCipher, macros::create_struct};

create_struct!(AffineCipherEncode);

impl AffineCipher for AffineCipherEncode {}

impl Operation for AffineCipherEncode {
    fn new(input: Request) -> Self {
        AffineCipherEncode {
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
                true => get_by_index(alp.0, <Self as AffineCipher>::encode(a, b, c, alp.0)),
                false => get_by_index(
                    alp.0,
                    <Self as AffineCipher>::encode(a, b, c.to_lowercase().next().unwrap(), alp.0),
                )
                .to_uppercase()
                .next()
                .unwrap(),
            });
        }

        Ok(plaintext)
    }

    fn validate(&self) -> Result<(), Error> {
        <Self as AffineCipher>::validate(&self.request)
    }
}
