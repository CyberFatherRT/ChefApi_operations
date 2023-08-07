use super::Operation;
use crate::{
    error::Error,
    libs::{AffineCipher, SwitchCase},
    macros::create_struct,
    utils::get_char_by_index,
};

create_struct!(AffineCipherEncode);

impl AffineCipher for AffineCipherEncode {}

impl Operation for AffineCipherEncode {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        AffineCipherEncode {
            name: "Affine Cipher Decode",
            module: "Cipher",
            description: Some("The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter."),
            info_url: Some("https://wikipedia.org/wiki/Affine_cipher"),
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        let (a, b) = <Self as AffineCipher>::get_a_b(&self.params);
        let (mut plaintext, alp) =
            <Self as AffineCipher>::get_plaintext_alp(&self.input, &self.lang);

        for c in self.input.chars() {
            if !c.is_alphabetic() {
                plaintext.push(c);
                continue;
            }

            plaintext.push(match c.is_lowercase() {
                true => get_char_by_index(alp.0, <Self as AffineCipher>::encode(a, b, c, alp.0)),
                false => get_char_by_index(
                    alp.0,
                    <Self as AffineCipher>::encode(a, b, c.to_lower_case(), alp.0),
                )
                .to_upper_case(),
            });
        }

        Ok(plaintext)
    }

    fn validate(&self) -> Result<(), Error> {
        <Self as AffineCipher>::validate(&self.lang, &self.params, &self.input)
    }
}
