use regex::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    traits::CharTrait,
    utils::{get_char_by_index, modulus, SupportedLanguage, EN_ALP, RU_ALP, RU_ALP_WITH_YO},
};

pub trait VigenereCipher {
    fn cipher<F>(lang: SupportedLanguage, key: &str, input: &str, f: F) -> Result<String, String>
    where
        F: Fn(i16, i16) -> i16,
    {
        <Self as VigenereCipher>::validate_language(&lang, key, input)?;

        let (alp, _, _, reg) = match lang {
            SupportedLanguage::En => EN_ALP,
            SupportedLanguage::Ru => RU_ALP,
            SupportedLanguage::RuWithYo => RU_ALP_WITH_YO,
        };

        let map: HashMap<char, usize> =
            HashMap::from_iter(alp.chars().enumerate().map(|(idx, elem)| (elem, idx)));

        let key = key.to_lowercase();
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
                .get(&get_char_by_index(&key, index % key_len))
                .unwrap()
                .to_owned() as i16;

            let text_idx = match c.is_lowercase() {
                true => map.get(&c).unwrap(),
                false => map.get(&c.to_lower_case()).unwrap(),
            }
            .to_owned() as i16;

            let idx = f(text_idx, key_idx);

            let plain_char = get_char_by_index(alp, modulus(idx, alp_len));
            cipher_text.push(match c.is_lowercase() {
                true => plain_char,
                false => plain_char.to_upper_case(),
            });

            index += 1;
        }
        Ok(cipher_text)
    }

    fn validate_language(lang: &SupportedLanguage, key: &str, input: &str) -> Result<(), String> {
        if input.is_empty() {
            return Err("Input is empty".to_string());
        };

        let reg = match lang {
            SupportedLanguage::En => Regex::new(EN_ALP.3).unwrap(),
            SupportedLanguage::Ru => Regex::new(RU_ALP.3).unwrap(),
            SupportedLanguage::RuWithYo => Regex::new(RU_ALP_WITH_YO.3).unwrap(),
        };

        if !reg.is_match(key) {
            return Err("Invalid key.".to_string());
        }

        Ok(())
    }
}
