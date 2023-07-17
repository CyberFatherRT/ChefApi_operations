macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}

macro_rules! regex_check {
    ($regex:tt == $string:expr) => {{
        let regex = regex::Regex::new($regex).unwrap();
        regex.is_match($string)
    }}
}

macro_rules! create_struct {
    ($name:ident) => {
        pub struct $name {
           name: &'static str,
           module: &'static str,
           description: Option<&'static str>,
           infoURL: Option<&'static str>,
           request: Request,
        }
    };
}

pub(crate) use {map, regex_check, create_struct};