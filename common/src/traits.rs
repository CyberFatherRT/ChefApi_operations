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