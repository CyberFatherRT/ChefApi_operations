use super::macros::map;
use num::{Integer, cast::ToPrimitive};

pub const EN_ALP: (&str, &str) = ("abcdefghijklmnopqrstuvwxyz", r"^[a-zA-Z]+$");
pub const RU_ALP: (&str, &str) = ("абвгдежзийклмнопрстуфхцчшщъыьэюя", "^[а-яА-Я]+$");
pub const RU_ALP_WITH_YO: (&str, &str) = ("абвгдеёжзийклмнопрстуфхцчшщъыьэюя", r"^[а-яА-ЯёЁ]+$");


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

pub fn egcd<T: Copy + Integer>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inv<T: Copy + Integer>(a: T, m: T) -> Option<T> {
    let (g, x, _) = egcd(a, m);
    if g != T::one() {
        None
    } else {
        Some((x % m + m) % m)
    }
}

pub fn get_index<T>(text: &str, index: T) -> char
    where T: Integer + ToPrimitive
{
    text.chars().nth(index.to_usize().unwrap()).unwrap()
}