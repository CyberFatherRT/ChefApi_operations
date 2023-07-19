use crate::api::{
    error::Error,
    macros::regex_check,
    operations::Request,
    utils::{get_index_by_char, validate_lang, NUM},
};
use num::traits::Euclid;
use unicode_segmentation::UnicodeSegmentation;

pub trait AffineCipher {
    fn encode(a: isize, b: isize, x: char, alp: &str) -> isize {
        let Ex =
            ((a * get_index_by_char(alp, x)) + b).rem_euclid(alp.graphemes(true).count() as isize);
        return Ex;
    }

    fn validate(request: &Request) -> Result<(), Error> {
        if request.params.len() != 2 {
            return Err(Error::InvalidNumberOfParamsError);
        }

        if !validate_lang(&request.input, &request.lang) {
            return Err(Error::UnsupportedLanguageError);
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
