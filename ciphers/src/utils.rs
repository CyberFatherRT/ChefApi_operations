use crate::{map, regex_check, traits::StringTrait};
use encoding_rs::UTF_8_INIT;
use itertools::Itertools;
use num::{FromPrimitive, Integer, ToPrimitive};
use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

// region constants

#[derive(Deserialize)]
pub enum SupportedLanguage {
    #[serde(alias = "en")]
    En,
    #[serde(alias = "ru")]
    Ru,
    #[serde(alias = "ru_with_yo")]
    RuAlpWithYo,
}

struct _AlphabetOptions {
    name: &'static str,
    value: &'static str,
}

const _ALPHABET_OPTIONS: &[_AlphabetOptions] = &[
    _AlphabetOptions {
        name: "Standard (RFC 4648): A-Za-z0-9+/=",
        value: "A-Za-z0-9+/=",
    },
    _AlphabetOptions {
        name: "URL safe (RFC 4648 §5): A-Za-z0-9-_",
        value: "A-Za-z0-9-_",
    },
    _AlphabetOptions {
        name: "Filename safe: A-Za-z0-9+-=",
        value: "A-Za-z0-9+\\-=",
    },
    _AlphabetOptions {
        name: "itoa64: ./0-9A-Za-z=",
        value: "./0-9A-Za-z=",
    },
    _AlphabetOptions {
        name: "y64: A-Za-z0-9._-",
        value: "A-Za-z0-9._-",
    },
    _AlphabetOptions {
        name: "z64: 0-9a-zA-Z+/=",
        value: "0-9a-zA-Z+/=",
    },
    _AlphabetOptions {
        name: "Radix-64 (RFC 4880): 0-9A-Za-z+/=",
        value: "0-9A-Za-z+/=",
    },
    _AlphabetOptions {
        name: "Uuencoding: [space]-_",
        value: " -_",
    },
    _AlphabetOptions {
        name: "Xxencoding: +-0-9A-Za-z",
        value: "+\\-0-9A-Za-z",
    },
    _AlphabetOptions {
        name: "BinHex: !-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
        value: "!-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
    },
    _AlphabetOptions {
        name: "ROT13: N-ZA-Mn-za-m0-9+/=",
        value: "N-ZA-Mn-za-m0-9+/=",
    },
    _AlphabetOptions {
        name: "UNIX crypt: ./0-9A-Za-z",
        value: "./0-9A-Za-z",
    },
    _AlphabetOptions {
        name: "Atom128: /128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
        value: "/128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
    },
    _AlphabetOptions {
        name: "Megan35: 3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
        value: "3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
    },
    _AlphabetOptions {
        name: "Zong22: ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
        value: "ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
    },
    _AlphabetOptions {
        name: "Hazz15: HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
        value: "HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
    },
];

#[derive(Eq, PartialEq)]
pub enum DataRepresentation {
    String(String),
    ByteArray(Vec<u8>),
}

pub const EN_ALP: (&str, &str, u8, &str) = (
    "abcdefghijklmnopqrstuvwxyz",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    26,
    r"^[a-zA-Z]+$",
);
pub const RU_ALP: (&str, &str, u8, &str) = (
    "абвгдежзийклмнопрстуфхцчшщъыьэюя",
    "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
    33,
    "^[а-яА-Я]+$",
);
pub const RU_ALP_WITH_YO: (&str, &str, u8, &str) = (
    "абвгдеёжзийклмнопрстуфхцчшщъыьэюя",
    "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
    33,
    r"^[а-яА-ЯёЁ]+$",
);
pub const NUM: (&str, &str) = ("0123456789", r"^\+?(0|[1-9]\d*)$");

// endregion

// region work with strings

pub fn expand_alphabet_range(alphabet: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let alphabet_length = alphabet.graphemes(true).count();
    let mut i = 0;

    while i < alphabet_length {
        let by_index = get_char_by_index(alphabet, i);
        if (i < alphabet_length - 2)
            && (get_char_by_index(alphabet, i + 1) == '-')
            && (by_index != '\\')
        {
            let (start, end) = (ord(by_index), ord(get_char_by_index(alphabet, i + 2)));

            for j in start..=end {
                result.push(chr(j));
            }
            i += 2;
        } else if (i < alphabet_length - 2)
            && (by_index == '\\')
            && (get_char_by_index(alphabet, i + 1) == '-')
        {
            result.push('-');
            i += 1;
        } else {
            result.push(by_index);
        }
        i += 1;
    }

    result
}

pub fn str_to_array_buffer(string: &str) -> Vec<u32> {
    if string.is_empty() {
        return Vec::new();
    }

    let string_length = string.graphemes(true).count();
    let mut result: Vec<u32> = vec![0; string_length];

    for (idx, elem) in result.iter_mut().enumerate() {
        *elem = ord(get_char_by_index(string, idx));
    }

    result
}

pub fn str_to_array_buffer_by_alphabet(string: &str, alphabet: &str) -> Vec<usize> {
    if string.is_empty() {
        return Vec::new();
    }

    let string_length = string.graphemes(true).count();
    let mut result: Vec<usize> = vec![0; string_length];
    for (idx, c) in string.chars().enumerate() {
        result[idx] = get_index_by_char(alphabet, c);
    }
    result
}

pub fn byte_array_to_chars(byte_array: Vec<u8>) -> Result<String, String> {
    String::from_utf8(byte_array).map_err(|err| err.to_string())
}

pub fn convert_to_byte_string(string: &str, convert_type: &str) -> Result<String, String> {
    match &*convert_type.to_lowercase() {
        "binary" => match from_binary(string, None, None) {
            Ok(data) => byte_array_to_chars(data),
            Err(e) => Err(e.to_string()),
        },
        "hex" => match from_hex(string, None, None) {
            Ok(data) => byte_array_to_chars(data),
            Err(e) => Err(e.to_string()),
        },
        "decimal" => match from_decimal(string, None) {
            Ok(data) => {
                let mut new_data = Vec::with_capacity(data.len());
                for elem in data.iter() {
                    match u8::from_usize(*elem) {
                        Some(val) => new_data.push(val),
                        None => return Err("a".to_string()),
                    };
                }
                byte_array_to_chars(new_data)
            }
            Err(e) => Err(e.to_string()),
        },
        "base64" => match from_base64(
            string.to_string(),
            "",
            DataRepresentation::ByteArray(Vec::new()),
            true,
            false,
        ) {
            Ok(data) => {
                let DataRepresentation::ByteArray(data) = data else {
                    unreachable!()
                };
                byte_array_to_chars(data)
            }
            Err(e) => Err(e.to_string()),
        },
        "utf8" | "utf-8" => match String::from_utf8(UTF_8_INIT.encode(string).0.into()) {
            Ok(data) => Ok(data),
            Err(e) => Err(e.to_string()),
        },
        "latin1" => unimplemented!(),
        _ => Ok(String::new()),
    }
}

pub fn from_binary(
    data: &str,
    delim: Option<&str>,
    byte_len: Option<usize>,
) -> Result<Vec<u8>, String> {
    if byte_len.unwrap_or(8) < 1 {
        return Err("Byte length must be a positive integer".to_string());
    };

    let delim = char_repr(delim.unwrap_or("Space"));
    let data = data.replace(delim, " ");

    let mut output: Vec<u8> = Vec::new();
    for i in data.split_whitespace() {
        match u8::from_str_radix(i, 2) {
            Ok(data) => output.push(data),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(output)
}

pub fn to_hex(data: &[u8]) -> String {
    data.iter()
        .fold(String::new(), |out, x| format!("{out}{:x}", x))
}

pub fn from_hex(
    data: &str,
    delim: Option<&str>,
    byte_len: Option<usize>,
) -> Result<Vec<u8>, String> {
    if byte_len.unwrap_or(8) < 1 {
        return Err("Byte length must be a positive integer".to_string());
    }

    let mut output: Vec<u8> = Vec::new();
    let delim = char_repr(delim.unwrap_or("Space"));

    for i in data.split(&delim) {
        match u8::from_str_radix(i, 16) {
            Ok(data) => output.push(data),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(output)
}

pub fn from_decimal(data: &str, delim: Option<&str>) -> Result<Vec<usize>, String> {
    let mut output = Vec::new();
    for i in data.split(char_repr(delim.unwrap_or("Space"))) {
        match i.parse::<usize>() {
            Ok(data) => output.push(data),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(output)
}

pub fn to_base64(data: &str, alphabet: Option<String>) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let alphabet = alphabet.unwrap_or("A-Za-z0-9+/=".to_string());

    let alphabet = expand_alphabet_range(&alphabet).iter().collect::<String>();

    let alphabet_length = alphabet.graphemes(true).count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err("Invalid base64 alphabet length".to_string());
    }

    let mut output = String::new();
    let mut padding = 0;

    for i in str_to_array_buffer(data)
        .iter()
        .fold(String::new(), |acc, x| acc + &format!("{:08b}", x))
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
        output.push(get_char_by_index(&alphabet, i))
    }

    output.push_str(&match alphabet_length {
        65 => get_char_by_index(&alphabet, 64).to_string().repeat(padding),
        _ => "".to_string(),
    });

    Ok(output)
}
pub fn from_base64(
    mut data: String,
    mut alphabet: &str,
    return_type: DataRepresentation,
    remove_non_alphabetic_chars: bool,
    strict_mode: bool,
) -> Result<DataRepresentation, String> {
    if data.is_empty() {
        return match return_type {
            DataRepresentation::String(_) => Ok(DataRepresentation::String(String::new())),
            DataRepresentation::ByteArray(_) => Ok(DataRepresentation::ByteArray(Vec::new())),
        };
    }

    if alphabet.is_empty() {
        alphabet = "A-Za-z0-9+/=";
    }

    if !remove_non_alphabetic_chars {
        let regex = regex::Regex::new(&format!("[^{}]", alphabet)).unwrap();
        if regex.is_match(&data) {
            return Err("Input string isn't correspond to used base64 alphabet.".to_string());
        }
    }

    let alphabet = expand_alphabet_range(alphabet).iter().collect::<String>();
    let alphabet_length = alphabet.graphemes(true).count();

    if alphabet_length != 64 && alphabet_length != 65 {
        return Err("Invalid base64 alphabet length".to_string());
    }

    if remove_non_alphabetic_chars {
        data = data.replace_by_alphabet(&alphabet)
    }

    if strict_mode {
        if data.len() % 4 == 1 {
            return Err(format!(
                "Invalid Base64 input length ({}) cannot be 4n+1, even without padding chars.",
                data.len()
            ));
        }

        if alphabet_length == 65 {
            let pad = get_char_by_index(&alphabet, 64);
            let pad_pos = data.find(pad);

            if let Some(pad_pos) = pad_pos {
                if pad_pos < data.len() - 2 || get_char_by_index(&data, data.len() - 1) != pad {
                    return Err(
                        "Base64 padding character ({pad}) not used in the correct place."
                            .to_string(),
                    );
                }

                if data.len() % 4 != 0 {
                    return Err("Base64 not padded to a multiple of 4.".to_string());
                }
            }
        }
    }

    if alphabet_length == 65 {
        data = data
            .trim_end_matches(get_char_by_index(&alphabet, 64))
            .to_string();
    }

    return match return_type {
        DataRepresentation::String(_) => {
            let mut output = String::new();
            str_to_array_buffer_by_alphabet(&data, &alphabet)
                .iter()
                .map(|x| format!("{:08b}", x)[2..].to_string())
                .collect::<String>()
                .chars()
                .chunks(8)
                .into_iter()
                .map(|x| u8::from_str_radix(&x.collect::<String>(), 2).unwrap() as char)
                .for_each(|x| output.push(x));

            Ok(DataRepresentation::String(output))
        }
        DataRepresentation::ByteArray(_) => {
            let mut output = Vec::new();

            str_to_array_buffer_by_alphabet(&data, &alphabet)
                .iter()
                .map(|x| format!("{:08b}", x)[2..].to_string())
                .collect::<String>()
                .chars()
                .chunks(8)
                .into_iter()
                .map(|x| u8::from_str_radix(&x.collect::<String>(), 2).unwrap())
                .for_each(|x| output.push(x));

            Ok(DataRepresentation::ByteArray(output))
        }
    };
}

pub fn validate_lang(text: &str, lang: &SupportedLanguage) -> bool {
    let re = match lang {
        SupportedLanguage::En => r"^[a-zA-Z\p{P}\s\d]+$",
        SupportedLanguage::Ru => r"^[а-яА-Я\p{P}\s\d]+$",
        SupportedLanguage::RuAlpWithYo => r"^[а-яА-ЯёЁ\p{P}\s\d]+$",
    };
    regex_check!(re => text)
}

pub fn get_alphabet(lang: &SupportedLanguage) -> (&'static str, &'static str, u8, &'static str) {
    match lang {
        SupportedLanguage::En => EN_ALP,
        SupportedLanguage::Ru => RU_ALP,
        SupportedLanguage::RuAlpWithYo => RU_ALP_WITH_YO,
    }
}

pub fn get_char_by_index<T: Integer + ToPrimitive>(text: &str, index: T) -> char {
    text.chars().nth(index.to_usize().unwrap()).unwrap()
}

pub fn get_index_by_char(text: &str, ch: char) -> usize {
    text.chars().position(|c| c == ch).unwrap()
}

pub fn char_repr(token: &str) -> &str {
    map!(
        "Space" => " ",
        "Percent" => "%",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "Tab" => "\t",
        "Line feed" => "\n",
        "CRLF" => "\r\n",
        "Forward slash" => "/",
        "Backslash" => "\\",
        "0x" => "0x",
        "\\x" => "\\x",
        "Nothing (separate chars)" => "",
        "None" => "",
    )
    .get(token)
    .unwrap_or(&" ")
}

pub fn chr(code: u32) -> char {
    char::from_u32(code).unwrap()
}

pub fn ord(chr: char) -> u32 {
    chr as u32
}

// endregion

// region math

pub fn update_step<T: Integer + Copy>(a: &mut T, old_a: &mut T, quotient: T) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_gcd<T: Integer + Copy>(a: T, b: T) -> (T, T, T) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coefficient_s) = (T::one(), T::zero());
    let (mut old_t, mut coefficient_t) = (T::zero(), T::one());

    while rem != T::zero() {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coefficient_s, &mut old_s, quotient);
        update_step(&mut coefficient_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}

pub fn mod_inv<T: Integer + Copy>(a: T, module: T) -> T {
    let (_, x, _) = extended_gcd(a, module);

    if x < T::zero() {
        x + module
    } else {
        x
    }
}

pub fn modulus<T: Integer + Copy>(x: T, y: T) -> T {
    ((x % y) + y) % y
}

pub fn add(a: i16, b: i16) -> i16 {
    a + b
}

pub fn sub(a: i16, b: i16) -> i16 {
    a - b
}

// endregion
