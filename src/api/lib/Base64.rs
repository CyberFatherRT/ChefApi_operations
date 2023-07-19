use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

use crate::api::{
    error::Error,
    utils::{expandAlphabetRange, getCharByIndex, strToArrayBuffer},
};

pub fn toBase64(data: &str, mut alphabet: &str) -> Result<String, Error> {
    if data.is_empty() {
        return Ok(String::new());
    }
    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=";
    }

    let alphabet = expandAlphabetRange(&alphabet).iter().collect::<String>();

    let alphabet_length = alphabet.graphemes(true).count();
    println!("{}", alphabet_length);
    if alphabet_length != 64 && alphabet_length != 65 {
        return Err(Error::Error {
            error: "Invalid base64 alphabet length",
        });
    }

    let mut output = String::new();
    let mut padding = 0;

    for i in strToArrayBuffer(data)
        .iter()
        .map(|x| format!("{:08b}", x))
        .collect::<String>()
        .chars()
        .chunks(6)
        .into_iter()
        .map(|x| {
            let sextet = x.collect::<String>();
            match sextet.len() {
                6 => u8::from_str_radix(&sextet, 2),
                _ => {
                    padding += 1;
                    u8::from_str_radix(&format!("{:0<6}", sextet), 2)
                }
            }
            .unwrap()
        })
    {
        output.push(getCharByIndex(&alphabet, i))
    }

    output.push_str(&match alphabet_length {
        65 => getCharByIndex(&alphabet, 64).to_string().repeat(padding),
        _ => "".to_string(),
    });

    Ok(output)
}
