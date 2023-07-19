use super::{Operation, Request};
use crate::api::{error::Error, lib::toBase64, macros::create_struct};

create_struct!(ToBase64);

impl Operation for ToBase64 {
    fn new(input: Request) -> Self {
        ToBase64 {
            name: "To Base64",
            module: "Default",
            description: Some("Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation encodes raw data into an ASCII Base64 string.<br><br>e.g. <code>hello</code> becomes <code>aGVsbG8=</code>"),
            infoURL: Some("https://wikipedia.org/wiki/Base64"),
            request: input,
        }
    }

    fn run(&self) -> Result<String, Error> {
        if let Err(e) = self.validate() {
            return Err(e);
        }
        toBase64(&self.request.input, &self.request.params[0])
    }

    fn validate(&self) -> Result<(), Error> {
        if self.request.params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.",
            });
        }

        Ok(())
    }
}
