use std::collections::BTreeMap;

use serde::Deserialize;

use crate::utils::{get_alphabet, SupportedLanguages};

pub struct BaconCipher {
    item_a: char,
    item_b: char,
    map: BTreeMap<char, String>,
}

impl BaconCipher {
    pub fn new(
        item_a: char,
        item_b: char,
        translation: SupportedBaconTranslation,
        alp: SupportedBaconAlphabet,
        lang: SupportedLanguages,
    ) -> Self {
        let (alphabet, _, _, _, length, _) = get_alphabet(&lang);

        let map = alphabet
            .chars()
            .map(|c| {
                Self::char_by_alphabet(c, alphabet, &translation, &alp, &length, &item_a, &item_b)
            })
            .collect();

        BaconCipher {
            item_a,
            item_b,
            map,
        }
    }

    pub fn encode(&self, elem: String) -> String {
        elem.chars()
            .map(|x| self.map.get(&x).unwrap_or(&x.to_string()).clone() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }

    pub fn a(&self) -> char {
        self.item_a
    }

    pub fn b(&self) -> char {
        self.item_b
    }
}

impl BaconCipher {
    fn char_by_alphabet(
        c: char,
        alphabet: &str,
        translation: &SupportedBaconTranslation,
        alp: &SupportedBaconAlphabet,
        length: &u8,
        a: &char,
        b: &char,
    ) -> (char, String) {
        let c = match alp {
            SupportedBaconAlphabet::Standard => {
                if c == 'j' {
                    'i'
                } else if c == 'u' {
                    'v'
                } else {
                    c
                }
            }
            SupportedBaconAlphabet::Complete => c,
        };

        let string = match (*length as f64).sqrt().round() as i8 {
            5 => format!("{:5b}", alphabet.find(c).unwrap()),
            6 => format!("{:6b}", alphabet.find(c).unwrap()),
            _ => unreachable!(),
        }
        .chars()
        .map(|x| match translation {
            SupportedBaconTranslation::ZeroOne => x,
            SupportedBaconTranslation::AB => match x {
                '0' => *a,
                '1' => *b,
                _ => unreachable!(),
            },
        })
        .collect();

        (c, string)
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
