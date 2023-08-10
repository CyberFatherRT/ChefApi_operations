use common::{create_operation_struct, error::Error, utils::to_base64, Operation};

create_operation_struct!(ToBase64);

impl Operation for ToBase64 {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        ToBase64 {
            name: "To Base64",
            module: "Default",
            description_en: Some("Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation encodes raw data into an ASCII Base64 string.<br><br>e.g. <code>hello</code> becomes <code>aGVsbG8=</code>"),
            description_ru: Some("Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция кодирует необработанные данные в строку ASCII Base64.<br><br>Например: <code>привет</code> becomes <code>h/ECHEMoaxC</code>"),
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
                error: "Invalid number of params.".to_string(),
            });
        }

        Ok(())
    }
}
