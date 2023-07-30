use crate::{
    error::Error,
    macros::regex_check,
    utils::{get_alphabet, get_index_by_char, mod_inv, modulus, validate_lang, NUM},
};
use unicode_segmentation::UnicodeSegmentation;

pub trait AffineCipher {
    fn encode(a: isize, b: isize, x: char, alp: &str) -> isize {
        let m = alp.graphemes(true).count() as isize;
        modulus((a * get_index_by_char(alp, x) as isize) + b, m)
    }

    fn decode(a: isize, b: isize, y: char, alp: &str) -> isize {
        let m = alp.graphemes(true).count() as isize;
        let inv_a = mod_inv(a, alp.graphemes(true).count() as isize);
        modulus(inv_a * (get_index_by_char(alp, y) as isize - b), m)
    }

    fn get_a_b(params: &[String]) -> (isize, isize) {
        return (
            params.get(0).unwrap().parse().unwrap(),
            params.get(1).unwrap().parse().unwrap(),
        );
    }

    fn get_plaintext_alp(input: &str, lang: &str) -> (String, (&'static str, &'static str)) {
        return (
            String::with_capacity(input.graphemes(true).count()),
            get_alphabet(lang),
        );
    }

    fn validate(lang: &str, params: &Vec<String>, input: &str) -> Result<(), Error> {
        if params.len() != 2 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.",
            });
        }

        if input.is_empty() {
            return Err(Error::InvalidInputError {
                error: "Input is empty.",
            });
        }

        if !validate_lang(input, lang) {
            return Err(Error::UnsupportedLanguageError {
                error: "Invalid language.",
            });
        }

        let (a, b) = (params.get(0).unwrap(), params.get(1).unwrap());

        if !regex_check!(NUM.1 => a) || !regex_check!(NUM.1 => b) {
            return Err(Error::InvalidParamTypeError {
                error: "The values of a and b can only be integers.",
            });
        };

        match a.parse::<usize>() {
            Ok(a) => a,
            Err(_) => {
                return Err(Error::InvalidParamTypeError {
                    error: "The values of a and b can only be integers.",
                });
            }
        };

        Ok(())
    }
}
