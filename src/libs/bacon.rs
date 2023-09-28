use serde::{Serialize, Deserialize};

const AlphabetStandatd: BaconAlphabet = BaconAlphabet {
    alphabet: "ABCDEFGHIKLMNOPQRSTUWXYZ",
    codes: Some([0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 19, 20, 21, 22, 23])
};

const AlphabetComplete: BaconAlphabet = BaconAlphabet {
    alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    codes: None,
};

#[derive(Serialize, Deserialize)]
pub struct BaconAlphabet {
    alphabet: &'static str,
    codes: Option<[isize; 26]>
}
