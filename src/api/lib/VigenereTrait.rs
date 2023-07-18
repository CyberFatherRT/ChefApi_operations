use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::api::{
    error::Error,
    operations::Request,
    utils::{get_index, EN_ALP, RU_ALP, RU_ALP_WITH_YO},
};

pub trait VigenereCipher {
    fn cipher<F: Fn(i16, i16) -> i16>(request: &Request, f: F) -> Result<String, Error> {
        if let Err(e) = <Self as VigenereCipher>::validate(&request.lang, &request.params) {
            return Err(e);
        }

        let (alp, reg) = match &*request.lang {
            "en" => EN_ALP,
            "ru" => RU_ALP,
            "ru_with_yo" => RU_ALP_WITH_YO,
            _ => unreachable!(),
        };

        let mut map = std::collections::HashMap::new();
        for (k, v) in alp.chars().enumerate() {
            map.insert(v, k);
        }

        let key = &request.params[0].to_lowercase();
        let rg = Regex::new(reg).unwrap();
        let mut index = 0usize;
        let mut cipher_text = String::new();

        let key_len = key.graphemes(true).count();
        let alp_len = alp.graphemes(true).count() as i16;

        for c in request.input.chars() {
            if !rg.is_match(&c.to_string()) {
                cipher_text.push(c);
                continue;
            }

            let key_idx = map
                .get(&get_index(key, index % key_len))
                .unwrap()
                .to_owned() as i16;

            let text_idx = match c.is_lowercase() {
                true => map.get(&c).unwrap(),
                false => map.get(&c.to_lowercase().next().unwrap()).unwrap(),
            }.to_owned() as i16;

            let idx = f(text_idx, key_idx);

            cipher_text.push(match c.is_lowercase() {
                true => get_index(alp, idx.rem_euclid(alp_len)),
                false => get_index(alp, idx.rem_euclid(alp_len))
                    .to_uppercase()
                    .next()
                    .unwrap(),
            });

            index += 1;
        }

        Ok(cipher_text)
    }

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
