use crate::{
    libs::affine_trait::AffineCipher, traits::SwitchCase, utils::get_char_by_index, Operation,
};
use num::Integer;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

pub struct AffineCipherInfo {
    pub name: &'static str,
    pub module: &'static str,
    pub description_en: Option<&'static str>,
    pub description_ru: Option<&'static str>,
    pub info_url: Option<&'static str>,
}

impl AffineCipherInfo {
    pub fn new() -> Self {
        Self {
            name: "Affine Cipher Decode",
            module: "Cipher",
            description_en: Some("The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter."),
            description_ru: Some("Аффинный шифр — это тип моноалфавитного шифра замены. Чтобы расшифровать, каждая буква в алфавите сопоставляется с ее числовым эквивалентом, расшифровывается с помощью математической функции и преобразуется обратно в букву."),
            info_url: Some("https://wikipedia.org/wiki/Affine_cipher"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DeserializeMeDaddy {
    a: isize,
    b: isize,
    lang: String,
    input: String,
}

#[derive(Serialize, Deserialize)]
struct AffineCipherDecode {
    request: String,
}

impl AffineCipher for AffineCipherDecode {}

impl Operation<DeserializeMeDaddy> for AffineCipherDecode {
    fn new(request: String) -> Self {
        Self { request }
    }

    fn run(&self) -> Result<String, String> {
        let request = self.validate()?;

        let (a, b) = (request.a, request.b);
        let (mut plaintext, alp) =
            <Self as AffineCipher>::get_plaintext_alp(&request.input, &request.lang);

        for c in self.input.chars() {
            if !c.is_alphabetic() {
                plaintext.push(c);
                continue;
            }

            plaintext.push(match c.is_lowercase() {
                true => get_char_by_index(alp.0, <Self as AffineCipher>::decode(a, b, c, alp.0)),
                false => get_char_by_index(
                    alp.0,
                    <Self as AffineCipher>::decode(a, b, c.to_lower_case(), alp.0),
                )
                .to_upper_case(),
            });
        }

        Ok(plaintext)
    }

    fn validate(&self) -> Result<DeserializeMeDaddy, String> {
        // <Self as AffineCipher>::validate(&self.lang, &self.params, &self.input)?;
        //
        // if self
        //     .params
        //     .get(0)
        //     .unwrap()
        //     .parse::<isize>()
        //     .unwrap()
        //     .gcd(&(self.lang.graphemes(true).count() as isize))
        //     != 1
        // {
        //     return Err("The value of `a` must be coprime to alphabet length.".to_string());
        // }
        serde_json::from_str(&self.request).map_err(|err| err.to_string())
    }
}
