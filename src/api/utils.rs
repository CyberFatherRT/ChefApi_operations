#![allow(dead_code)]

use super::macros::{map, regex_check};
use num::{Integer, ToPrimitive};
use unicode_segmentation::UnicodeSegmentation;

// region constants

pub const EN_ALP: (&str, &str) = ("abcdefghijklmnopqrstuvwxyz", r"^[a-zA-Z]+$");
pub const RU_ALP: (&str, &str) = ("абвгдежзийклмнопрстуфхцчшщъыьэюя", "^[а-яА-Я]+$");
pub const RU_ALP_WITH_YO: (&str, &str) = ("абвгдеёжзийклмнопрстуфхцчшщъыьэюя", r"^[а-яА-ЯёЁ]+$");
pub const NUM: (&str, &str) = ("0123456789", r"^\+?(0|[1-9]\d*)$");

// endregion

// region work with strings

pub fn expandAlphabetRange(alphabet: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let alphabet_length = alphabet.graphemes(true).count();
    let mut i = 0;

    while i < alphabet_length {
        let by_index = getCharByIndex(alphabet, i);
        if i < alphabet_length - 2 && getCharByIndex(alphabet, i + 1) == '-' && by_index != '\\' {
            let (start, end) = (ord(by_index), ord(getCharByIndex(alphabet, i + 2)));

            for j in start..=end {
                result.push(chr(j));
            }
            i += 2;
        } else if i < alphabet_length - 2
            && by_index == '\\'
            && getCharByIndex(alphabet, i + 1) != '-'
        {
            result.push('-');
            i += 1;
        } else {
            result.push(by_index);
            i += 1;
        }
    }

    return result;
}

pub fn strToArrayBuffer(string: &str) -> Vec<u32> {
    if string.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<u32> = Vec::with_capacity(string.len());
    let mut i = string.len();

    while i > 0 {
        result[i] = ord(getCharByIndex(string, i));
        i -= 1;
    }
    result
}

pub fn validateLang(text: &str, lang: &str) -> bool {
    let re = match lang {
        "en" => r"^[a-zA-Z\p{P}\s\d]+$",
        "ru" => r"^[а-яА-Я\p{P}\s\d]+$",
        "ru_with_yo" => r"^[а-яА-ЯёЁ\p{P}\s\d]+$",
        _ => return false,
    };
    regex_check!(re => text)
}

pub fn getAlphabet(lang: &str) -> (&str, &str) {
    match lang {
        "en" => EN_ALP,
        "ru" => RU_ALP,
        "ru_with_yo" => RU_ALP_WITH_YO,
        _ => EN_ALP,
    }
}

pub fn getCharByIndex<T: Integer + ToPrimitive>(text: &str, index: T) -> char {
    text.chars().nth(index.to_usize().unwrap()).unwrap()
}

pub fn getIndexByChar(text: &str, ch: char) -> isize {
    text.chars().position(|c| c == ch).unwrap() as isize
}

pub fn charRepr(token: &str) -> &str {
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
