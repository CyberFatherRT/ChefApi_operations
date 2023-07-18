use super::{Operation, Request};
use crate::api::{
    error::Error,
    macros::{regex_check, create_struct},
    utils::char_rep,
};

create_struct!(A1Z26CipherDecode);

impl Operation for A1Z26CipherDecode {
    fn new(input: Request) -> Box<Self> {
        Box::new(A1Z26CipherDecode {
            name: "A1Z26 Cipher Decode",
            module: "Cipher",
            description: Some("Converts alphabet order numbers into their corresponding  alphabet character.<br><br>e.g. <code>1</code> becomes <code>a</code> and <code>2</code> becomes <code>b</code>."),
            infoURL: None,
            request: input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        let delimiter = char_rep(self.request.params.get(0).unwrap());

        let cipher_text = self.request.input
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
}

impl A1Z26CipherDecode {
    fn validate(&self) -> Result<(), Error> {
        if self.request.input.is_empty() {
            return Err(Error::InvalidInputError {
                error: "Input is empty",
            });
        }

        let regex_checked = match self.request.params[0].as_str() {
            "Space" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]) ?)+\s*$" == &self.request.input)
            }
            "Comma" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]),?)+\s*$" == &self.request.input)
            }
            "Semi-colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]);?)+\s*$" == &self.request.input)
            }
            "Colon" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6]):?)+\s*$" == &self.request.input)
            }
            "Line feed" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\n?)+\s*$" == &self.request.input)
            }
            "CRLF" => {
                regex_check!(r"^\s*(([1-9]|1[0-9]|2[0-6])\r\n?)+\s*$" == &self.request.input)
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