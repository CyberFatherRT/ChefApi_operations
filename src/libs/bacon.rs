use std::collections::HashMap;

use num::integer::Roots;
use serde::Deserialize;

use crate::utils::{get_alphabet, SupportedLanguages, EN_ALP};

struct BaconCipher {
    item_a: char,
    item_b: char,
    map: HashMap<char, String>,
}

impl BaconCipher {
    pub fn new(
        item_a: char,
        item_b: char,
        translation: SupportedBaconTranslation,
        alphabet: SupportedBaconAlphabet,
        lang: SupportedLanguages,
    ) -> Self {
        let (alp, _, _, _, length, _) = get_alphabet(&lang);
        let length = length.sqrt();

        let map: HashMap<char, String> = alp
            .chars()
            .map(|c| {
                let c = char_by_alphabet(&c, &alphabet);
                (c, format!("{c}:05b"))
            })
            .collect();

        BaconCipher {
            item_a,
            item_b,
            map,
        }
    }
}

impl BaconCipher {
    pub fn encode(&self, elem: &char) -> String {
        todo!()
    }

    pub fn a(&self) -> char {
        self.item_a.clone()
    }

    pub fn b(&self) -> char {
        self.item_b.clone()
    }
}

impl Default for BaconCipher {
    fn default() -> Self {
        BaconCipher::new(
            'A',
            'B',
            SupportedBaconTranslation::AB,
            SupportedBaconAlphabet::Standard,
            SupportedLanguages::EN,
        )
    }
}

fn char_by_alphabet(c: &char, alp: &SupportedBaconAlphabet) -> char {
    match alp {
        SupportedBaconAlphabet::Standard => {
            if *c == 'j' {
                'i'
            } else if *c == 'u' {
                'v'
            } else {
                *c
            }
        }
        SupportedBaconAlphabet::Complete => *c,
    };

    todo!()
}

#[derive(Deserialize)]
pub enum SupportedBaconTranslation {
    #[serde(rename = "0/1")]
    ZeroOne,
    #[serde(rename = "A/B")]
    AB,
}

#[derive(Deserialize)]
pub enum SupportedBaconAlphabet {
    #[serde(rename = "Standard (I=J and V=U)")]
    Standard,
    Complete,
}
