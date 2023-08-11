use crate::error::Error;
use common::{create_operation_struct, regex_check, Operation};

create_operation_struct!(AnalyseHash);

impl Operation for AnalyseHash {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        Self {
            name: "Analyse Hash",
            module: "Crypto",
            description_en: Some("Tries to determine information about a given hash and suggests which algorithm may have been used to generate it based on its length."),
            description_ru: Some("Пытается определить информацию о заданном хэше и предлагает, какой алгоритм мог быть использован для его генерации, исходя из его длины."),
            info_url: Some("https://wikipedia.org/wiki/Comparison_of_cryptographic_hash_functions"),
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;
        let input: String = self
            .input
            .chars()
            .map(|x| match x {
                '\t' | '\n' | ' ' => "".to_string(),
                _ => x.to_string(),
            })
            .collect::<String>();

        let byte_length: f64 = input.len() as f64 / 2.0;
        let bit_length: f64 = byte_length * 8.0;
        let output: String = format!(
            r#"Hash length: {hash_length}
Byte length: {byte_length}
Bit length: {bit_length}


Based on the length, this hash could have been generated by one of the following hashing fuctions:

"#,
            hash_length = input.len()
        );

        let possible_hash_functions = match bit_length as usize {
            4 => vec!["Fletcher-4", "Luhn algorithm", "Verhoeff algorithm"],
            8 => vec!["Fletcher-8"],
            16 => vec!["BSD checksum", "CRC-16", "SYSV checksum", "Fletcher-16"],
            32 => vec!["CRC-32", "Fletcher-32", "Adler-32"],
            64 => vec!["CRC-64", "RIPEMD-64", "SipHash"],
            128 => vec![
                "MD5",
                "MD4",
                "MD2",
                "HAVAL-128",
                "RIPEMD-128",
                "Snefru",
                "Tiger-128",
            ],
            160 => vec![
                "SHA-1",
                "SHA-0",
                "FSB-160",
                "HAS-160",
                "HAVAL-160",
                "RIPEMD-160",
                "Tiger-160",
            ],
            192 => vec!["Tiger", "HAVAL-192"],
            224 => vec!["SHA-224", "SHA3-224", "ECOH-224", "FSB-224", "HAVAL-224"],
            256 => vec![
                "SHA-256",
                "SHA3-256",
                "BLAKE-256",
                "ECOH-256",
                "FSB-256",
                "GOST",
                "Grøstl-256",
                "HAVAL-256",
                "PANAMA",
                "RIPEMD-256",
                "Snefru",
            ],
            320 => vec!["RIPEMD-320"],
            384 => vec!["SHA-384", "SHA3-384", "ECOH-384", "FSB-384"],
            512 => vec![
                "SHA-512",
                "SHA3-512",
                "BLAKE-512",
                "ECOH-512",
                "FSB-512",
                "Grøstl-512",
                "JH",
                "MD6",
                "Spectral Hash",
                "SWIFFT",
                "Whirlpool",
            ],
            1024 => vec!["Fowler-Noll-Vo"],
            _ => {
                vec!["Unknown"]
            }
        };
        println!("{:?}", possible_hash_functions);
        Ok(format!("{output}{}", possible_hash_functions.join("\n")))
    }

    fn validate(&self) -> Result<(), Error> {
        if !regex_check!(r"" => &self.input) {
            return Err(Error::Error {
                error: String::from("Invalid hash"),
            });
        }

        Ok(())
    }
}
