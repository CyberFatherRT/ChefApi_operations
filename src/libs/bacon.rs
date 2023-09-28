use serde::{Serialize, Deserialize};

static ALPHABET_STANDATD: BaconAlphabet = BaconAlphabet {
    alphabet: "ABCDEFGHIKLMNOPQRSTUWXYZ",
    codes: Some([0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 19, 20, 21, 22, 23])
};

static ALPHABET_COMPLETE: BaconAlphabet = BaconAlphabet {
    alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    codes: None,
};

static BACON_TRANSLATION_01: &str = "0/1";
static BACON_TRANSLATION_AB: &str = "A/B";
static BACON_TRANSLATION_CASE: &str = "Case";
static BACON_TRANSLATION_AMNZ: &str = "A-M/N-Z first letter";

static BACON_TRANSLATIONS: [&str; 4] = [
    BACON_TRANSLATION_01,
    BACON_TRANSLATION_AB,
    BACON_TRANSLATION_CASE,
    BACON_TRANSLATION_AMNZ,
];

static BACON_TRANSLATIONS_FOR_ENCODING: [&str; 2] = [
    BACON_TRANSLATION_01,
    BACON_TRANSLATION_AB
];

pub fn swap_zero_and_one(string: &str) -> String {
    string.chars().map(|x| {
        match x {
            '0' => '1',
            '1' => '0',
            x => x,
        }
    }).collect()
}

#[derive(Serialize, Deserialize)]
pub struct BaconAlphabet {
    alphabet: &'static str,
    codes: Option<[isize; 26]>
}
