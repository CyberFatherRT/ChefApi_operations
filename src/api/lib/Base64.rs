use crate::api::utils::expandAlphabetRange;
use unicode_segmentation::UnicodeSegmentation;

pub fn toBase64(data: &str, mut alphabet: &str) -> String {
    if data.is_empty() {
        return String::new();
    }

    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=";
    }

    alphabet = expandAlphabetRange(alphabet).join(" ");
    let alphabet_length = alphabet.graphemes(true).count();
    let mut output = String::new();
    String::new()
}
