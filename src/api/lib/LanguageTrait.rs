use regex::Regex;

use crate::api::{
    Request,
    error::Error,
    utils::{EN_ALP, RU_ALP, RU_ALP_WITH_YO}
};

pub trait SupportedLanguages {
    fn validate(request: &Request) -> Result<(), Error> {
        let langs = ["en", "ru", "ru_with_yo"];

        if !langs.contains(&&*request.lang.clone()) {
            return Err(Error::UnsupportedLanguageError);
        }

        if request.params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError);
        }

        let reg = match request.lang.as_str() {
            "en" => Regex::new(EN_ALP.1).unwrap(),
            "ru" => Regex::new(RU_ALP.1).unwrap(),
            "ru_with_yo" => Regex::new(RU_ALP_WITH_YO.1).unwrap(),
            _ => return Err(Error::UnsupportedLanguageError),
        };

        if !reg.is_match(&request.params[0]) {
            return Err(Error::IvalidKeyError);
        }

        Ok(())
    }
}