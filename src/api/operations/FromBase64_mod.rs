use super::{Operation, Request};
use crate::api::{error::Error, lib::fromBase64, macros::create_struct};

create_struct!(FromBase64);

impl Operation for FromBase64 {
    fn new(input: Request) -> Self {
        FromBase64 {
            name: "From Base64",
            module: "Default",
            description: Some("Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation encodes raw data into an ASCII Base64 string.<br><br>e.g. <code>hello</code> becomes <code>aGVsbG8=</code>"),
            infoURL: Some("https://wikipedia.org/wiki/Base64"),
            request: input,
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        match fromBase64(
            self.request.input.clone(),
            &self.request.params[0],
            matches!(&*self.request.params[1], "true"),
            matches!(&*self.request.params[1], "true")
        ) {
            Ok(output) => Ok(output.trim_end_matches('\0').to_string()),
            Err(e) => Err(e),
        }
    }

    fn validate(&self) -> Result<(), Error> {
        if self.request.params.len() != 3 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.",
            });
        }

        Ok(())
    }
}
