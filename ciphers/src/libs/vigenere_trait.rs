use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use common::{
    error::Error,
    traits::SwitchCase,
    utils::{get_char_by_index, modulus, EN_ALP, RU_ALP, RU_ALP_WITH_YO},
};

pub trait VigenereCipher {
    fn cipher<F>(lang: &str, params: &Vec<String>, input: &str, f: F) -> Result<String, Error>
    where
        F: Fn(i16, i16) -> i16,
    {
        <Self as VigenereCipher>::validate_language(lang, params, input)?;

        let (alp, reg) = match lang {
            "en" => EN_ALP,
            "ru" => RU_ALP,
            "ru_with_yo" => RU_ALP_WITH_YO,
            _ => unreachable!(),
        };

        let mut map = std::collections::HashMap::new();
        for (k, v) in alp.chars().enumerate() {
            map.insert(v, k);
        }

        let key = &params[0].to_lowercase();
        let rg = Regex::new(reg).unwrap();
        let mut index = 0usize;
        let mut cipher_text = String::new();

        let key_len = key.graphemes(true).count();
        let alp_len = alp.graphemes(true).count() as i16;

        for c in input.chars() {
            if !rg.is_match(&c.to_string()) {
                cipher_text.push(c);
                continue;
            }

            let key_idx = map
                .get(&get_char_by_index(key, index % key_len))
                .unwrap()
                .to_owned() as i16;

            let text_idx = match c.is_lowercase() {
                true => map.get(&c).unwrap(),
                false => map.get(&c.to_lower_case()).unwrap(),
            }
            .to_owned() as i16;

            let idx = f(text_idx, key_idx);

            cipher_text.push(match c.is_lowercase() {
                true => get_char_by_index(alp, modulus(idx, alp_len)),
                false => get_char_by_index(alp, modulus(idx, alp_len)).to_upper_case(),
            });

            index += 1;
        }

        Ok(cipher_text)
    }

    fn validate_language(lang: &str, params: &Vec<String>, input: &str) -> Result<(), Error> {
        if input.is_empty() {
            return Err(Error::InvalidInputError {
                error: "Input is empty".to_string(),
            });
        }

        let langs = ["en", "ru", "ru_with_yo"];

        if !langs.contains(&lang) {
            return Err(Error::UnsupportedLanguageError {
                error: "Unsupported language.".to_string(),
            });
        }

        if params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params error.".to_string(),
            });
        }

        let reg = match lang {
            "en" => Regex::new(EN_ALP.1).unwrap(),
            "ru" => Regex::new(RU_ALP.1).unwrap(),
            "ru_with_yo" => Regex::new(RU_ALP_WITH_YO.1).unwrap(),
            _ => {
                return Err(Error::UnsupportedLanguageError {
                    error: "Unsupported language.".to_string(),
                })
            }
        };

        if !reg.is_match(&params[0]) {
            return Err(Error::IvalidKeyError {
                error: "invalid key.".to_string(),
            });
        }

        Ok(())
    }
}
