use crate::{
    traits::CharTrait,
    utils::{
        get_alphabet, get_char_by_index, get_index_by_char, modulus, validate_lang,
        SupportedLanguage,
    },
};
use num::Integer;

pub fn affine_cipher_encode(
    input: &str,
    lang: SupportedLanguage,
    a: i16,
    b: i16,
) -> Result<String, String> {
    if !validate_lang(input, &lang) {
        return Err("Wrong language.".to_string());
    };

    let (alp_lower, alp_upper, alp_length, _) = get_alphabet(&lang);
    if a.gcd(&(alp_length as i16)) != 1 {
        return Err(format!(
            "The value of `a` must be coprime to alphabet length({}).",
            alp_length
        ));
    }

    let mut output = String::with_capacity(alp_length as usize);

    for c in input.chars() {
        if !c.is_alphabetic() {
            output.push(c);
            continue;
        }

        let x = match c.is_lowercase() {
            true => get_index_by_char(alp_lower, c),
            false => get_index_by_char(alp_upper, c),
        } as i16;

        let x = modulus(a * x + b, alp_length as i16);

        output.push(match c.is_lowercase() {
            true => get_char_by_index(alp_lower, x),
            false => get_char_by_index(alp_upper, x).to_upper_case(),
        });
    }

    Ok(output)
}
