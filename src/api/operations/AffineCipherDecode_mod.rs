use unicode_segmentation::UnicodeSegmentation;

use super::{Operation, Request};
use crate::api::utils::get_by_index;
use crate::api::{
    error::Error,
    macros::{create_struct, regex_check},
    utils::{extended_gcd, get_alphabet, get_index_by_char, mod_inv, validate_lang, NUM},
};

create_struct!(AffineCipherDecode);

impl Operation for AffineCipherDecode {
    fn new(input: Request) -> Box<Self> {
        Box::new(AffineCipherDecode {
            name: "Affine Cipher Decode",
            module: "Cipher",
            description: Some("The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter."),
            infoURL: Some("https://wikipedia.org/wiki/Affine_cipher"),
            request: input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        let (a, b) = (
            self.request
                .params
                .get(0)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
            self.request
                .params
                .get(1)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
        );

        let mut plaintext = String::new();
        let alp = get_alphabet(&self.request.lang);
        let alp_len = alp.0.graphemes(true).count() as isize;
        let aModInv = mod_inv(a, alp.0.len() as isize);

        for c in self.request.input.chars() {
            if !c.is_alphabetic() {
                plaintext.push(c);
                continue;
            }

            let (index, flag) = match c.is_lowercase() {
                true => ((get_index_by_char(alp.0, c) - b) * aModInv, true),
                false => (
                    (get_index_by_char(alp.0, c.to_lowercase().next().unwrap()) - b) * aModInv,
                    false,
                ),
            };

            plaintext.push(match flag {
                true => get_by_index(alp.0, index.rem_euclid(alp_len)),
                false => get_by_index(alp.0, index.rem_euclid(alp_len))
                    .to_uppercase()
                    .next()
                    .unwrap(),
            });
        }

        Ok(plaintext)
    }

    fn validate(&self) -> Result<(), Error> {
        if self.request.params.len() != 2 {
            return Err(Error::InvalidNumberOfParamsError);
        }

        if !validate_lang(&self.request.input, &self.request.lang) {
            return Err(Error::UnsupportedLanguageError);
        }

        let (a, b) = (
            self.request.params.get(0).unwrap(),
            self.request.params.get(1).unwrap(),
        );

        if !regex_check!(NUM.1 => a) || !regex_check!(NUM.1 => b) {
            return Err(Error::InvalidParamTypeError {
                error: "The values of a and b can only be integers.",
            });
        };

        let a = match a.parse::<usize>() {
            Ok(a) => a,
            Err(_) => {
                return Err(Error::InvalidParamTypeError {
                    error: "The values of a and b can only be integers.",
                });
            }
        };

        let alp = get_alphabet(&self.request.lang);

        if extended_gcd(a, alp.0.len()).0 != 1 {
            return Err(Error::InvalidParamTypeError {
                error: "The value of 'a' must be coprime to the length of the alphabet.",
            });
        }

        Ok(())
    }
}
