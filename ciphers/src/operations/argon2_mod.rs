use crate::Operation;
use argon2::{Config, ThreadMode, Variant, Version};
use common::utils::{from_base64, to_hex, DataRepresentation};
use serde::Deserialize;

// region info about operation

pub struct Argon2Info {
    pub name: &'static str,
    pub module: &'static str,
    pub description_en: Option<&'static str>,
    pub description_ru: Option<&'static str>,
    pub info_url: Option<&'static str>,
}

impl Argon2Info {
    pub fn new() -> Self {
        Self {
            name: "Argon2",
            module: "Crypto",
            description_en: Some("Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.<br><br>Enter the password in the input to generate its hash."),
            description_ru: Some("Argon2 – это функция получения ключа, которая была выбрана победителем конкурса хеширования паролей в июле 2015 года. Она была разработана Алексом Бирюковым, Даниэлем Дину и Дмитрием Ховратовичем из Люксембургского университета.<br><br>Введите пароль в ввод для генерации его хэша."),
            info_url: Some("https://wikipedia.org/wiki/Argon2"),
        }
    }
}

// endregion

// region structs and enums

#[derive(Deserialize)]
#[serde(remote = "Variant")]
enum MyVariant {
    Argon2d = 0,
    Argon2i = 1,
    Argon2id = 2,
}

#[derive(Deserialize)]
enum OutputFormat {
    Encoded,
    Hex,
    Raw,
}

#[derive(Deserialize)]
struct Argon2Params {
    salt: String,
    iterations: u32,
    memory: u32,
    parallelism: u32,
    hash_length: u32,
    #[serde(with = "MyVariant")]
    argon2_type: Variant,
    output_format: OutputFormat,
}

#[derive(Deserialize)]
pub struct DeserializeMeDaddy {
    input: String,
    params: Argon2Params,
}

pub struct Argon2 {
    request: String,
}

// endregion

impl Operation<DeserializeMeDaddy> for Argon2 {
    fn new(request: String) -> Self {
        Self { request }
    }

    fn run(&self) -> Result<String, String> {
        let request = self.validate()?;

        let (params, input) = (request.params, request.input);

        let config = Config {
            variant: params.argon2_type,
            version: Version::Version13,
            mem_cost: params.memory,
            time_cost: params.iterations,
            lanes: params.parallelism,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: params.hash_length,
        };

        let hash = argon2::hash_encoded(input.as_bytes(), params.salt.as_bytes(), &config).unwrap();

        let output = match params.output_format {
            OutputFormat::Encoded => hash,
            format => {
                let raw_hash = hash.split('$').nth(5).unwrap();

                let data = match from_base64(
                    raw_hash.to_string(),
                    "",
                    DataRepresentation::String(String::new()),
                    false,
                    false,
                ) {
                    Ok(DataRepresentation::String(data)) => data,
                    _ => unreachable!(),
                };

                match format {
                    OutputFormat::Hex => to_hex(data.as_bytes()),
                    OutputFormat::Raw => data,
                    _ => unreachable!(),
                }
            }
        };

        Ok(output)
    }

    fn validate(&self) -> Result<DeserializeMeDaddy, String> {
        serde_json::from_str(&self.request).map_err(|err| err.to_string())
    }
}
