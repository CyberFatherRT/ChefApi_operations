use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::api::{
    error::Error,
    operations::Request,
    utils::{get_index, EN_ALP, RU_ALP, RU_ALP_WITH_YO},
};
use super::SupportedLanguages;

pub trait VigenereCipher: SupportedLanguages {
    fn cipher<F>(request: &Request, f: F) -> Result<String, Error>
        where F: Fn(i16, i16) -> i16
    {
        if let Err(e) = <Self as SupportedLanguages>::validate(&request) {
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
}
