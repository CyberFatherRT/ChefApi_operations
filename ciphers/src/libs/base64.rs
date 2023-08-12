use crate::traits::StringTrait;
use crate::utils::{
    expand_alphabet_range, get_char_by_index, str_to_array_buffer, str_to_array_buffer_by_alphabet,
    DataRepresentation,
};
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

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
    println!("here?");
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
