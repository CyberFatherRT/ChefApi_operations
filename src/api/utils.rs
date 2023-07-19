use num::{traits::Euclid, Integer, ToPrimitive};

use super::macros::{map, regex_check};

pub const EN_ALP: (&str, &str) = ("abcdefghijklmnopqrstuvwxyz", r"^[a-zA-Z]+$");
pub const RU_ALP: (&str, &str) = ("абвгдежзийклмнопрстуфхцчшщъыьэюя", "^[а-яА-Я]+$");
pub const RU_ALP_WITH_YO: (&str, &str) = ("абвгдеёжзийклмнопрстуфхцчшщъыьэюя", r"^[а-яА-ЯёЁ]+$");
pub const NUM: (&str, &str) = ("0123456789", r"^\+?(0|[1-9]\d*)$");

pub fn validate_lang(text: &str, lang: &str) -> bool {
    let re = match lang {
        "en" => r"^[a-zA-Z\p{P}\s\d]+$",
        "ru" => r"^[а-яА-Я\p{P}\s\d]+$",
        "ru_with_yo" => r"^[а-яА-ЯёЁ\p{P}\s\d]+$",
        _ => return false,
    };
    regex_check!(re => text)
}

pub fn char_rep(token: &str) -> &str {
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

pub fn get_alphabet(lang: &str) -> (&str, &str) {
    match lang {
        "en" => EN_ALP,
        "ru" => RU_ALP,
        "ru_with_yo" => RU_ALP_WITH_YO,
        _ => EN_ALP,
    }
}

pub fn update_step<T>(a: &mut T, old_a: &mut T, quotient: T)
where
    T: Integer + Copy,
{
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_gcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Integer + Copy,
{
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (T::one(), T::zero());
    let (mut old_t, mut coeff_t) = (T::zero(), T::one());

    while rem != T::zero() {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
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

pub fn modulus<T: Integer + Euclid>(a: T, m: T) -> T {
    a.rem_euclid(&m)
}

pub fn get_by_index<T: Integer + ToPrimitive>(text: &str, index: T) -> char {
    text.chars().nth(index.to_usize().unwrap()).unwrap()
}

pub fn get_index_by_char(text: &str, ch: char) -> isize {
    text.chars().position(|c| c == ch).unwrap() as isize
}

pub fn add(a: i16, b: i16) -> i16 {
    a + b
}

pub fn sub(a: i16, b: i16) -> i16 {
    a - b
}
