pub trait SwitchCase {
    fn to_lower_case(&self) -> Self;
    fn to_upper_case(&self) -> Self;
}

impl SwitchCase for char {
    fn to_lower_case(&self) -> char {
        match self {
            'A'..='Z' => self.to_ascii_lowercase(),
            'А'..='Я' => char::from_u32(*self as u32 + 32).unwrap(),
            'Ё' => char::from_u32(*self as u32 + 80).unwrap(),
            _ => *self,
        }
    }

    fn to_upper_case(&self) -> char {
        match self {
            'a'..='z' => self.to_ascii_lowercase(),
            'а'..='я' => char::from_u32(*self as u32 - 32).unwrap(),
            'ё' => char::from_u32(*self as u32 - 80).unwrap(),
            _ => *self,
        }
    }
}

pub trait RegexReplace {
    fn replace_all(&self, regex: &str, replacement: &str) -> Result<String, String>;
    fn replace(&self, regex: &str, replacement: &str) -> Result<String, String>;
}

impl RegexReplace for String {
    fn replace_all(&self, regex_str: &str, replacement: &str) -> Result<String, String> {
        let Ok(re) = regex::Regex::new(regex_str) else {
            return Err(String::from("wrong regex"));
        };

        let output: String = re.replace_all(regex_str, replacement).to_string();
        Ok(output)
    }

    fn replace(&self, regex_str: &str, replacement: &str) -> Result<String, String> {
        let Ok(re) = regex::Regex::new(regex_str) else {
            return Err(String::from("wrong regex"));
        };

        let output: String = re.replace(regex_str, replacement).to_string();
        Ok(output)
    }
}
