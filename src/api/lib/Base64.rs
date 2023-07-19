use crate::api::{
    error::Error,
    utils::{expandAlphabetRange, getCharByIndex, strToArrayBuffer},
};
use unicode_segmentation::UnicodeSegmentation;

pub fn toBase64(data: &str, mut alphabet: String) -> Result<String, Error> {
    if data.is_empty() {
        return Ok(String::new());
    }

    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=".to_string();
    }

    alphabet = String::from_iter(expandAlphabetRange(&alphabet));
    let alphabet_length = alphabet.graphemes(true).count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err(Error::Error {
            error: "Invalid base64 alphabet length",
        });
    }

    let data = strToArrayBuffer(data);
    let mut output = String::new();
    let (mut enc1, mut enc2, mut enc3, mut enc4) = (0u32, 0u32, 0u32, 0u32);
    let mut i = 0;

    while i < data.len() {
        let char1 = match data.get(i) {
            Some(x) => *x,
            None => 0u32,
        };
        i += 1;
        let char2 = match data.get(i) {
            Some(x) => *x,
            None => 0u32,
        };
        i += 1;
        let char3 = match data.get(i) {
            Some(x) => *x,
            None => 0u32,
        };
        i += 1;

        enc1 = char1 >> 4;
        enc2 = ((char1 & 0x03) << 4) | (char2 >> 4);
        enc3 = ((char2 & 0x0f) << 2) | (char3 >> 6);
        enc4 = char3 & 0x3f;

        if enc1 != 0 {
            output.push(getCharByIndex(&alphabet, enc1))
        }
        if enc2 != 0 {
            output.push(getCharByIndex(&alphabet, enc2))
        }
        if enc3 != 0 {
            output.push(getCharByIndex(&alphabet, enc3))
        }
        if enc4 != 0 {
            output.push(getCharByIndex(&alphabet, enc4))
        }
    }

    Ok(output)
}
