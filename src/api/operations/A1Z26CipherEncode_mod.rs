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
            request: input,
        })
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        let mut result = String::new();
        let delimiter = char_rep(&self.request.params[0]);

        for character in self.request.input.chars() {
            result.push_str(&*format!("{}{}", match character {
                'a'..='z' => character as u8 - 96,
                'A'..='Z' => character as u8 - 64,
                _ => { continue; }
            }, delimiter));
        }
        Ok(result.trim_end_matches(|c: char| &*c.to_string() == delimiter).to_string())
    }

    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
