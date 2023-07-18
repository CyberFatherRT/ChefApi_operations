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
    }};
}

macro_rules! create_struct {
    ($name:ident) => {
        pub struct $name {
            pub name: &'static str,
            pub module: &'static str,
            pub description: Option<&'static str>,
            pub infoURL: Option<&'static str>,
            pub request: Request,
        }
    };
}

pub(crate) use {create_struct, map, regex_check};
