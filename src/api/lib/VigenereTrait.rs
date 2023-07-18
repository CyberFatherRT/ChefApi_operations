use regex::Regex;

use crate::api::error::Error;
use crate::api::utils::{EN_ALP, RU_ALP, RU_ALP_WITH_YO};

pub trait VigenereCipher {
    fn validate(lang: &String, params: &Vec<String>) -> Result<(), Error> {
        let langs = ["en", "ru", "ru_with_yo"];

        if !langs.contains(&&*lang.clone()) {
            return Err(Error::UnsupportedLanguageError);
        }

        if params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError);
        }


        let reg = match lang.as_str() {
            "en" => Regex::new(EN_ALP.1).unwrap(),
            "ru" => Regex::new(RU_ALP.1).unwrap(),
            "ru_with_yo" => Regex::new(RU_ALP_WITH_YO.1).unwrap(),
            _ => return Err(Error::UnsupportedLanguageError),
        };

        if !reg.is_match(&params[0]) {
            return Err(Error::IvalidKeyError);
        }

        Ok(())
    }
}