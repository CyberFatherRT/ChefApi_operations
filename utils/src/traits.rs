use std::collections::HashSet;

pub trait CharTrait {
    fn to_lower_case(&self) -> Self;
    fn to_upper_case(&self) -> Self;
}

impl CharTrait for char {
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
            'a'..='z' => self.to_ascii_uppercase(),
            'а'..='я' => char::from_u32(*self as u32 - 32).unwrap(),
            'ё' => char::from_u32(*self as u32 - 80).unwrap(),
            _ => *self,
        }
    }
}

pub trait StringTrait {
    fn replace_by_alphabet(&self, alphabet: &str) -> String;

    fn regex_replace_all(&self, regex: &str, replacement: &str) -> Result<String, String>;
    fn regex_replace(&self, regex: &str, replacement: &str) -> Result<String, String>;

    fn capitalize(&self) -> String;
}

impl StringTrait for String {
    fn replace_by_alphabet(&self, alphabet: &str) -> String {
        let alphabet: HashSet<char> = HashSet::from_iter(alphabet.chars());

        self.chars()
            .filter(|c| alphabet.contains(c))
            .collect::<String>()
    }

    fn regex_replace_all(&self, regex_str: &str, replacement: &str) -> Result<String, String> {
        let Ok(re) = regex::Regex::new(regex_str) else {
            return Err(String::from("wrong regex"));
        };
        let output: String = re.replace_all(regex_str, replacement).to_string();
        Ok(output)
    }

    fn regex_replace(&self, regex_str: &str, replacement: &str) -> Result<String, String> {
        let Ok(re) = regex::Regex::new(regex_str) else {
            return Err(String::from("wrong regex"));
        };

        let output: String = re.replace(regex_str, replacement).to_string();
        Ok(output)
    }
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
