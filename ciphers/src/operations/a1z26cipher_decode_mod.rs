use super::Operation;
use crate::{
    error::Error,
    macros::{create_struct, regex_check},
    utils::char_repr,
};

create_struct!(A1Z26CipherDecode);

impl Operation for A1Z26CipherDecode {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        A1Z26CipherDecode {
            name: "A1Z26 Cipher Decode",
            module: "Cipher",
            description: Some("Converts alphabet order numbers into their corresponding  alphabet character.<br><br>e.g. <code>1</code> becomes <code>a</code> and <code>2</code> becomes <code>b</code>."),
            info_url: None,
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        let delimiter = char_repr(self.params.get(0).unwrap());

        let cipher_text = self
            .input
            .trim_matches(|c: char| [delimiter].contains(&&*c.to_string()))
            .split(delimiter);

        let mut plain_text = String::new();

        for c in cipher_text {
            let c = c.parse::<u32>().unwrap();
            if !(1..=26).contains(&c) {
                return Err(Error::OperationError {
                    error: "Error: all numbers must be between 1 and 26.",
                });
            }
            plain_text.push(char::from_u32(c + 96u32).unwrap());
        }
        Ok(plain_text)
    }

    fn validate(&self) -> Result<(), Error> {
        if self.input.is_empty() {
            return Err(Error::InvalidInputError {
                error: "Input is empty",
            });
        }

        let regex_checked = match self.params[0].as_str() {
            "Space" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]) ?)+\s*$" => &self.input)
            }
            "Comma" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]),?)+\s*$" => &self.input)
            }
            "Semi-colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]);?)+\s*$" => &self.input)
            }
            "Colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]):?)+\s*$" => &self.input)
            }
            "Line feed" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\n?)+\s*$" => &self.input)
            }
            "CRLF" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\r\n?)+\s*$" => &self.input)
            }
            _ => false,
        };
        if !regex_checked {
            return Err(Error::InvalidInputError {
                error: "Invalid input",
            });
        }

        Ok(())
    }
}
