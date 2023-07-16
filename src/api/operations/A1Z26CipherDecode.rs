use super::Operation;
use crate::api::error::Error;
use crate::api::utils::char_rep;

pub fn A1Z26CipherDecoder(request: Operation) -> Result<String, Error> {
    let binding = char_rep("Space").to_string();
    let separator = request.params.get(0).unwrap_or(&binding);

    let cipher_text = request.input;

    if cipher_text.is_empty() {
        return Err(Error::EmptyInputError);
    }

    let mut plain_text = String::new();

    for letter in cipher_text.split(separator) {
        if letter.len() > 2 || letter.len() < 1 {
            return Err(Error::InvalidInputError{error: "Wrong length"});
        }
        let letter = letter.parse::<u8>().unwrap();
        if letter < 1 || letter > 26 {
            return Err(Error::OperationError {
                error: "Error: all numbers must be between 1 and 26.",
            });
        }
        plain_text.push((letter + 96) as char);
    }
    Ok(plain_text)
}
