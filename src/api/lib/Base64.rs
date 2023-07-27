use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

use crate::api::{
    error::Error,
    utils::{
        expandAlphabetRange, getCharByIndex, regex_replace, strToArrayBuffer,
        strToArrayBufferByAlphabet,
    },
};

pub fn toBase64(data: &str, mut alphabet: &str) -> Result<String, Error> {
    if data.is_empty() {
        return Ok(String::new());
    }
    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=";
    }

    let alphabet = expandAlphabetRange(alphabet).iter().collect::<String>();

    let alphabet_length = alphabet.graphemes(true).count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err(Error::Error {
            error: "Invalid base64 alphabet length".to_string(),
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

pub fn fromBase64(
    mut data: String,
    mut alphabet: &str,
    removeNonAlpChars: bool,
    strictMode: bool,
) -> Result<String, Error> {
    if data.is_empty() {
        return Ok(String::new());
    }
    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=";
    }

    {
        let regex = regex::Regex::new(&format!("[{}]", alphabet)).unwrap();
        if !regex.is_match(&data) {
            return Err(Error::Error {
                error: "Invalid base64 alphabet".to_string(),
            });
        }
    }

    let alphabet = expandAlphabetRange(alphabet).iter().collect::<String>();

    let alphabet_length = alphabet.graphemes(true).count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err(Error::Error {
            error: "Invalid base64 alphabet length".to_string(),
        });
    }

    if removeNonAlpChars {
        let re = format!("[^{}]", regex_replace(&alphabet, r"[\[\]\\\-^$]", r"\$&"));
        data = regex_replace(&data, &re, "");
    }

    if strictMode {
        if data.len() % 4 == 1 {
            return Err(Error::Error {
                error: format!(
                    "Invalid Base64 input length ({}) cannot be 4n+1, even without padding chars.",
                    data.len()
                ),
            });
        }

        if alphabet_length == 65 {
            let pad = getCharByIndex(&alphabet, 64);
            let padPos = data.find(pad);

            if let Some(padPos) = padPos {
                if padPos < data.len() - 2 || getCharByIndex(&data, data.len() - 1) != pad
                {
                    return Err(Error::Error {
                        error: format!(
                            "Base64 padding character ({}) not used in the correct place.",
                            pad
                        ),
                    });
                }

                if data.len() % 4 != 0 {
                    return Err(Error::Error {
                        error: "Base64 not padded to a multiple of 4.".to_string(),
                    });
                }
            }
        }
    }

    if alphabet_length == 65 {
        data = data
            .trim_end_matches(getCharByIndex(&alphabet, 64))
            .to_string();
    }

    let mut output = String::new();

    for i in strToArrayBufferByAlphabet(&data, &alphabet)
        .iter()
        .map(|x| format!("{:08b}", x)[2..].to_string())
        .collect::<String>()
        .chars()
        .chunks(8)
        .into_iter()
        .map(|x| char::from_u32(u32::from_str_radix(&x.collect::<String>(), 2).unwrap()).unwrap())
    {
        output.push(i)
    }

    Ok(output)
}

struct AlphabetOptions {
    name: &'static str,
    value: &'static str,
}

const ALPHABET_OPTIONS: &[AlphabetOptions] = &[
    AlphabetOptions {
        name: "Standard (RFC 4648): A-Za-z0-9+/=",
        value: "A-Za-z0-9+/=",
    },
    AlphabetOptions {
        name: "URL safe (RFC 4648 §5): A-Za-z0-9-_",
        value: "A-Za-z0-9-_",
    },
    AlphabetOptions {
        name: "Filename safe: A-Za-z0-9+-=",
        value: "A-Za-z0-9+\\-=",
    },
    AlphabetOptions {
        name: "itoa64: ./0-9A-Za-z=",
        value: "./0-9A-Za-z=",
    },
    AlphabetOptions {
        name: "y64: A-Za-z0-9._-",
        value: "A-Za-z0-9._-",
    },
    AlphabetOptions {
        name: "z64: 0-9a-zA-Z+/=",
        value: "0-9a-zA-Z+/=",
    },
    AlphabetOptions {
        name: "Radix-64 (RFC 4880): 0-9A-Za-z+/=",
        value: "0-9A-Za-z+/=",
    },
    AlphabetOptions {
        name: "Uuencoding: [space]-_",
        value: " -_",
    },
    AlphabetOptions {
        name: "Xxencoding: +-0-9A-Za-z",
        value: "+\\-0-9A-Za-z",
    },
    AlphabetOptions {
        name: "BinHex: !-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
        value: "!-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
    },
    AlphabetOptions {
        name: "ROT13: N-ZA-Mn-za-m0-9+/=",
        value: "N-ZA-Mn-za-m0-9+/=",
    },
    AlphabetOptions {
        name: "UNIX crypt: ./0-9A-Za-z",
        value: "./0-9A-Za-z",
    },
    AlphabetOptions {
        name: "Atom128: /128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
        value: "/128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
    },
    AlphabetOptions {
        name: "Megan35: 3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
        value: "3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
    },
    AlphabetOptions {
        name: "Zong22: ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
        value: "ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
    },
    AlphabetOptions {
        name: "Hazz15: HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
        value: "HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
    },
];
