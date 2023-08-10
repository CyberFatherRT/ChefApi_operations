use argon2::{self, Config, ThreadMode, Variant, Version};
use common::utils::{from_base64, DataRepresentation};
use common::{
    create_struct,
    error::Error,
    utils::{convert_to_byte_string, to_hex},
    Operation,
};

create_struct!(Argon2);

impl Operation for Argon2 {
    fn new(lang: String, params: Vec<String>, input: String) -> Self {
        Self {
            name: "Argon2",
            module: "Crypto",
            description_en: Some("Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.<br><br>Enter the password in the input to generate its hash."),
            description_ru: Some("Argon2 – это функция получения ключа, которая была выбрана победителем конкурса хеширования паролей в июле 2015 года. Она была разработана Алексом Бирюковым, Даниэлем Дину и Дмитрием Ховратовичем из Люксембургского университета.<br><br>Введите пароль в ввод для генерации его хэша."),
            info_url: Some("https://wikipedia.org/wiki/Argon2"),
            lang,
            params,
            input
        }
    }

    fn run(&self) -> Result<String, Error> {
        self.validate()?;

        let (salt, time, mem, parallelism, hash_len, hash_type, out_format) = (
            match convert_to_byte_string(&self.params[0], &self.params[1]) {
                Ok(salt) => salt,
                Err(e) => {
                    return Err(Error::Error {
                        error: e.to_string(),
                    })
                }
            },
            match self.params[2].parse::<u32>() {
                Ok(time) => time,
                Err(_) => {
                    return Err(Error::Error {
                        error: "\"Time\" parameter must be integer.".to_string(),
                    })
                }
            },
            match self.params[3].parse::<u32>() {
                Ok(time) => time,
                Err(_) => {
                    return Err(Error::Error {
                        error: "\"Memory\" parameter must be integer.".to_string(),
                    })
                }
            },
            match self.params[4].parse::<u32>() {
                Ok(time) => time,
                Err(_) => {
                    return Err(Error::Error {
                        error: "\"Parallelism\" parameter must be integer.".to_string(),
                    })
                }
            },
            match self.params[5].parse::<u32>() {
                Ok(time) => time,
                Err(_) => {
                    return Err(Error::Error {
                        error: "\"HashLen\" parameter must be integer.".to_string(),
                    })
                }
            },
            match &*self.params[6] {
                "Argon2i" => Variant::Argon2i,
                "Argon2d" => Variant::Argon2d,
                "Argon2id" => Variant::Argon2id,
                _ => unreachable!(),
            },
            {
                if !["Encoded hash", "Hex hash", "Raw hash"].contains(&&*self.params[7]) {
                    return Err(Error::Error {
                        error: "Unsupported \"Output format\"".to_string(),
                    });
                }
                self.params[7].to_string()
            },
        );

        let config = Config {
            variant: hash_type,
            version: Version::Version13,
            mem_cost: mem,
            time_cost: time,
            lanes: parallelism,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: hash_len,
        };

        let hash = argon2::hash_encoded(self.input.as_bytes(), salt.as_bytes(), &config).unwrap();

        let output = match &*out_format {
            "Encoded hash" => hash,
            "Raw hash" => {
                let raw_hash = hash.split('$').nth(5).unwrap();

                let DataRepresentation::String(data) = from_base64(
                    raw_hash.to_string(),
                    "",
                    DataRepresentation::String(String::new()),
                    false,
                    false,
                )
                .unwrap() else {
                    return Err(Error::Error {
                        error: String::new(),
                    });
                };
                data
            }
            "Hex hash" => {
                let raw_hash = hash.split('$').nth(5).unwrap();

                let DataRepresentation::String(data) = from_base64(
                    raw_hash.to_string(),
                    "",
                    DataRepresentation::String(String::new()),
                    false,
                    false,
                )
                .unwrap() else {
                    return Err(Error::Error {
                        error: String::new(),
                    });
                };
                to_hex(data.as_bytes())
            }
            _ => unreachable!(),
        };

        Ok(output)
    }

    fn validate(&self) -> Result<(), Error> {
        if self.params.len() != 8 {
            return Err(Error::InvalidNumberOfParamsError {
                error: "Invalid number of params.".to_string(),
            });
        }

        if !["Argon2i", "Argon2d", "Argon2id"].contains(&&*self.params[6]) {
            return Err(Error::Error {
                error: "Invalid hash algorithm".to_string(),
            });
        }

        Ok(())
    }
}
