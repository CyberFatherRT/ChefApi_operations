use crate::api::{
    error::Error,
    macros::regex_check,
    operations::Request,
    utils::{getAlphabet, getIndexByChar, mod_inv, modulus, validateLang, NUM},
};
use unicode_segmentation::UnicodeSegmentation;

pub trait AffineCipher {
    fn encode(a: isize, b: isize, x: char, alp: &str) -> isize {
        let m = alp.graphemes(true).count() as isize;
        let Ex = modulus((a * getIndexByChar(alp, x) as isize) + b, m);
        return Ex;
    }

    fn decode(a: isize, b: isize, y: char, alp: &str) -> isize {
        let m = alp.graphemes(true).count() as isize;

        let inv_a = mod_inv(a, alp.graphemes(true).count() as isize);

        let Dy = modulus(inv_a * (getIndexByChar(alp, y) as isize - b), m);

        return Dy;
    }

    fn get_a_b(request: &Request) -> (isize, isize) {
        return (
            request.params.get(0).unwrap().parse::<isize>().unwrap(),
            request.params.get(1).unwrap().parse::<isize>().unwrap(),
        );
    }

    fn get_plaintext_alp(request: &Request) -> (String, (&str, &str)) {
        return (
            String::with_capacity(request.input.graphemes(true).count()),
            getAlphabet(&request.lang),
        );
    }

    fn validate(request: &Request) -> Result<(), Error> {
        if request.params.len() != 2 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.",
            });
        }

        if request.input.is_empty() {
            return Err(Error::InvalidInputError {
                error: "Input is empty.",
            });
        }

        if !validateLang(&request.input, &request.lang) {
            return Err(Error::UnsupportedLanguageError {
                error: "Invalid language.",
            });
        }

        let (a, b) = (
            request.params.get(0).unwrap(),
            request.params.get(1).unwrap(),
        );

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
