use common::{create_struct, error::Error, utils::char_repr, Operation};

create_struct!(A1Z26CipherEncode);

impl Operation for A1Z26CipherEncode {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        A1Z26CipherEncode {
            name: "A1Z26 Cipher Encode",
            module: "Cipher",
            description: Some("Converts alphabet characters into their corresponding alphabet order number.<br><br>e.g. <code>a</code> becomes <code>1</code> and <code>b</code> becomes <code>2</code>.<br><br>Non-alphabet characters are dropped."),
            info_url: None,
            lang,
            params,
            input,
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        if self.input.is_empty() {
            return Ok(String::new());
        }

        let mut result = String::new();
        let delimiter = char_repr(&self.params[0]);

        for character in self.input.chars() {
            result.push_str(&format!(
                "{}{}",
                match character {
                    'a'..='z' => character as u8 - 96,
                    'A'..='Z' => character as u8 - 64,
                    _ => continue,
                },
                delimiter
            ));
        }
        Ok(result[..result.len() - 1].to_string())
    }

    fn validate(&self) -> Result<(), Error> {
        if self.params.len() != 1 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.".to_string(),
            });
        }

        Ok(())
    }
}
