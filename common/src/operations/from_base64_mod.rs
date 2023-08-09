use crate::{error::Error, from_base64, macros::create_struct, Operation};

create_struct!(FromBase64);

impl Operation for FromBase64 {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        FromBase64 {
            name: "From Base64",
            module: "Default",
            description: Some("Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation encodes raw data into an ASCII Base64 string.<br><br>e.g. <code>hello</code> becomes <code>aGVsbG8=</code>"),
            info_url: Some("https://wikipedia.org/wiki/Base64"),
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        match from_base64(
            self.input.clone(),
            &self.params[0],
            matches!(&*self.params[1], "true"),
            matches!(&*self.params[1], "true"),
        ) {
            Ok(output) => Ok(output.trim_end_matches('\0').to_string()),
            Err(e) => Err(e),
        }
    }

    fn validate(&self) -> Result<(), Error> {
        if self.params.len() != 3 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.".to_string(),
            });
        }

        Ok(())
    }
}
