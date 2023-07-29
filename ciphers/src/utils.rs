use super::macros::{map, regex_check};
use num::{Integer, ToPrimitive};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

// region constants

pub const EN_ALP: (&str, &str) = ("abcdefghijklmnopqrstuvwxyz", r"^[a-zA-Z]+$");
pub const RU_ALP: (&str, &str) = ("абвгдежзийклмнопрстуфхцчшщъыьэюя", "^[а-яА-Я]+$");
pub const RU_ALP_WITH_YO: (&str, &str) = ("абвгдеёжзийклмнопрстуфхцчшщъыьэюя", r"^[а-яА-ЯёЁ]+$");
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

    for i in 0..result.len() {
        result[i] = ord(get_char_by_index(string, i));
    }

    result
}

pub fn str_to_array_buffer_by_alphabet(string: &str, alphabet: &str) -> Vec<u32> {
    if string.is_empty() {
        return Vec::new();
    }

    let string_length = string.graphemes(true).count();
    let mut result: Vec<u32> = vec![0; string_length];

    for (idx, c) in string.chars().enumerate() {
        result[idx] = get_index_by_char(alphabet, c) as u32;
    }

    result
}

pub fn regex_replace(string: &str, regex: &str, replacement: &str) -> String {
    let re = Regex::new(regex).unwrap();
    return re.replace_all(string, replacement).to_string();
}

pub fn validate_lang(text: &str, lang: &str) -> bool {
    let re = match lang {
        "en" => r"^[a-zA-Z\p{P}\s\d]+$",
        "ru" => r"^[а-яА-Я\p{P}\s\d]+$",
        "ru_with_yo" => r"^[а-яА-ЯёЁ\p{P}\s\d]+$",
        _ => return false,
    };
    regex_check!(re => text)
}

pub fn get_alphabet(lang: &str) -> (&'static str, &'static str) {
    match lang {
        "en" => EN_ALP,
        "ru" => RU_ALP,
        "ru_with_yo" => RU_ALP_WITH_YO,
        _ => EN_ALP,
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
        "Persent" => "%",
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
