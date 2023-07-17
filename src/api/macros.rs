macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = ::std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}

macro_rules! regex_check {
    ($regex:tt == $string:expr) => {{
        let regex = ::regex::Regex::new($regex).unwrap();
        regex.is_match($string)
    }}
}

pub(crate) use map;
pub(crate) use regex_check;