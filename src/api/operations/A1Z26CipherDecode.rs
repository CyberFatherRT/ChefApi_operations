use super::Operation;
use crate::api::error::Error;
use crate::api::utils::char_rep;

pub async fn A1Z26CipherDecoder(request: Operation) -> Result<String, Error> {
    let separator = char_rep(&request.params[0]);

    if request.input.is_empty() {
        return Err(Error::InvalidInputError {
            error: "Invalid length",
        });
    }

    let cipher_text = request.input.split(separator);
    let mut plain_text = String::new();

    for letter in cipher_text {
        if letter.len() != 1 {
            return Err(Error::InvalidInputError {
                error: "Invalid length",
            });
        }

        let letter = match letter.parse::<char>() {
            Ok(letter) => match letter.to_digit(10) {
                Some(letter) => letter,
                None => {
                    return Err(Error::InvalidInputError {
                        error: "Invalid letter",
                    });
                }
            },
            Err(_) => {
                return Err(Error::InvalidInputError {
                    error: "Invalid letter",
                });
            }
        };

        if letter < 1 || letter > 26 {
            return Err(Error::OperationError {
                error: "Error: all numbers must be between 1 and 26.",
            });
        }
        plain_text.push(char::from_u32(letter + 96u32).unwrap());
    }
    Ok(plain_text)
}
