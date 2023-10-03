use crate::{
    create_me_daddy,
    libs::bacon::{SupportedBaconAlphabet, SupportedBaconTranslation},
    utils::{get_alphabet, SupportedLanguages},
    Operation,
};
use serde::{Deserialize, Serialize};

impl Operation<'_, DeserializeMeDaddy, String> for BaconCipherEncode {
    fn do_black_magic(&self, request: &str) -> Result<String, String> {
        let request = self.validate(request)?;
        todo!()
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
