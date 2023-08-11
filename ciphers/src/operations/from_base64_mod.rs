use crate::error::Error;
use crate::utils::{from_base64, DataRepresentation};
use common::{create_operation_struct, Operation};

create_operation_struct!(FromBase64);

impl Operation for FromBase64 {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        FromBase64 {
            name: "From Base64",
            module: "Default",
            description_en: Some("Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.<br><br>e.g. <code>aGVsbG8=</code> becomes <code>hello</code>"),
            description_ru: Some("Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция декодирует необработанные данные в строку ASCII Base64.<br><br>e.g. <code>h/ECHEMoaxC</code> становиться <code>привет</code>"),
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
            DataRepresentation::String(String::new()),
            matches!(&*self.params[1], "true"),
            matches!(&*self.params[1], "true"),
        ) {
            Ok(output) => {
                let DataRepresentation::String(output) = output else {
                    unreachable!()
                };
                Ok(output.trim_end_matches('\0').to_string())
            }
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
