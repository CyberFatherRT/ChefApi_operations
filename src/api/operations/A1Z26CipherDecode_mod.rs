use super::{Operation, Request};
use crate::api::error::Error;
use crate::api::macros::regex_check;
use crate::api::utils::char_rep;

pub struct A1Z26CipherDecode {
    name: &'static str,
    module: &'static str,
    description: Option<&'static str>,
    infoURL: Option<&'static str>,
    input: Request,
}

impl Operation for A1Z26CipherDecode {
    fn new(input: Request) -> Box<Self> {
        Box::new(A1Z26CipherDecode {
            name: "A1Z26 Cipher Decode",
            module: "Cipher",
            description: Some("Converts alphabet order numbers into their corresponding  alphabet character.<br><br>e.g. <code>1</code> becomes <code>a</code> and <code>2</code> becomes <code>b</code>."),
            infoURL: None,
            input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.check() {
            return Err(e);
        }

        let delimiter = char_rep(self.input.params.get(0).unwrap());

        let cipher_text = self.input.input
            .trim_matches(|c: char| [delimiter].contains(&&*c.to_string()))
            .split(delimiter);

        let mut plain_text = String::new();

        for c in cipher_text {
            let c = c.parse::<u32>().unwrap();
            if c < 1 || c > 26 {
                return Err(Error::OperationError {
                    error: "Error: all numbers must be between 1 and 26."
                });
            }
            plain_text.push(char::from_u32(c + 96u32).unwrap());
        }
        Ok(plain_text)
    }

    fn check(&self) -> Result<(), Error> {
        if self.input.input.is_empty() {
            return Err(Error::InvalidInputError {
                error: "Input is empty",
            });
        }

        let regex_checked = match self.input.params[0].as_str() {
            "Space" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]) ?)+\s*$" == &self.input.input)
            }
            "Comma" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]),?)+\s*$" == &self.input.input)
            }
            "Semi-colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]);?)+\s*$" == &self.input.input)
            }
            "Colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]):?)+\s*$" == &self.input.input)
            }
            "Line feed" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\n?)+\s*$" == &self.input.input)
            }
            "CRLF" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\r\n?)+\s*$" == &self.input.input)
            }
            _ => false
        };
        if !regex_checked {
            return Err(Error::InvalidInputError {
                error: "Invalid input",
            });
        }

        Ok(())
    }
}
