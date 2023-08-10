use crate::libs::affine_trait::AffineCipher;
use common::{
    create_operation_struct, error::Error, traits::SwitchCase, utils::get_char_by_index, Operation,
};
use num::Integer;
use unicode_segmentation::UnicodeSegmentation;

create_operation_struct!(AffineCipherDecode);

impl AffineCipher for AffineCipherDecode {}

impl Operation for AffineCipherDecode {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        AffineCipherDecode {
            name: "Affine Cipher Decode",
            module: "Cipher",
            description_en: Some("The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter."),
            description_ru: Some("Аффинный шифр — это тип моноалфавитного шифра замены. Чтобы расшифровать, каждая буква в алфавите сопоставляется с ее числовым эквивалентом, расшифровывается с помощью математической функции и преобразуется обратно в букву."),
            info_url: Some("https://wikipedia.org/wiki/Affine_cipher"),
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        let (a, b) = <Self as AffineCipher>::get_a_b(&self.params);
        let (mut plaintext, alp) =
            <Self as AffineCipher>::get_plaintext_alp(&self.input, &self.lang);

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

    fn validate(&self) -> Result<(), Error> {
        <Self as AffineCipher>::validate(&self.lang, &self.params, &self.input)?;

        if self
            .params
            .get(0)
            .unwrap()
            .parse::<isize>()
            .unwrap()
            .gcd(&(self.lang.graphemes(true).count() as isize))
            != 1
        {
            return Err(Error::InvalidParamTypeError {
                error: "The value of `a` must be coprime to alphabet length.".to_string(),
            });
        }

        Ok(())
    }
}
