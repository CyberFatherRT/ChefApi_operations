use super::Operation;
use crate::{error::Error, libs::to_base64, macros::create_struct};

create_struct!(ToBase64);

impl Operation for ToBase64 {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        ToBase64 {
            name: "To Base64",
            module: "Default",
            description: Some("Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation encodes raw data into an ASCII Base64 string.<br><br>e.g. <code>hello</code> becomes <code>aGVsbG8=</code>"),
            info_url: Some("https://wikipedia.org/wiki/Base64"),
            lang,
            params,
            input,
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;
        to_base64(&self.input, &self.params[0])
    }

    fn validate(&self) -> Result<(), Error> {
        if self.params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.",
            });
        }

        Ok(())
    }
}
