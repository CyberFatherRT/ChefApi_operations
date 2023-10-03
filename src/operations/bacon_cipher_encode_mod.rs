use crate::{
    create_me_daddy,
    libs::bacon::{BaconCipher, SupportedBaconAlphabet, SupportedBaconTranslation},
    utils::SupportedLanguages,
    Operation,
};
use serde::{Deserialize, Serialize};
use std::mem::swap;

impl Operation<'_, DeserializeMeDaddy, String> for BaconCipherEncode {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        let (
            input,
            Params {
                bacon_alphabet,
                translation,
                keep_extra_character,
                invert_translation,
                lang,
            },
        ) = (request.input, request.params);

        let (mut a, mut b) = match translation {
            SupportedBaconTranslation::ZeroOne => ('0', '1'),
            SupportedBaconTranslation::AB => ('A', 'B'),
        };

        if invert_translation {
            swap(&mut a, &mut b);
        }

        let cipher = BaconCipher::new(a, b, translation, bacon_alphabet, lang);

        let output = cipher.encode(&input);

        let output = if keep_extra_character {
            output.join("")
        } else {
            output.iter().filter(|x| x.len() != 1).cloned().collect()
        };

        Ok(output)
    }
}

create_me_daddy!();

#[derive(Deserialize)]
struct Params {
    bacon_alphabet: SupportedBaconAlphabet,
    translation: SupportedBaconTranslation,
    keep_extra_character: bool,
    invert_translation: bool,
    lang: SupportedLanguages,
}

pub struct BaconCipherEncode;
