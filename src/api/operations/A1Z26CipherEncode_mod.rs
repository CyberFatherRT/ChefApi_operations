use super::{Operation, Request};
use crate::api::{
    macros::create_struct
};
use crate::api::error::Error;
use crate::api::utils::char_rep;

create_struct!(A1Z26CipherEncode);

impl Operation for A1Z26CipherEncode {
    fn new(input: Request) -> Box<Self> {
        Box::new(A1Z26CipherEncode {
            name: "A1Z26 Cipher Encode",
            module: "Cipher",
            description: Some("Converts alphabet characters into their corresponding alphabet order number.<br><br>e.g. <code>a</code> becomes <code>1</code> and <code>b</code> becomes <code>2</code>.<br><br>Non-alphabet characters are dropped."),
            infoURL: None,
            input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        let mut result = String::new();
        let delimiter = char_rep(&self.input.params[0]);

        for character in self.input.input.chars() {
            if character.is_ascii_alphabetic() {
                result.push_str(&*format!("{}{}", character, delimiter));
            }
        }
        Ok(result)
    }

    fn check(&self) -> Result<(), Error> {
        todo!()
    }
}