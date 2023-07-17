use super::macros::map;

pub fn char_rep(token: &str) -> &str {
    map!(
        "Space" => " ",
        "Persent" => "%",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "Tab" => "\t",
        "Line feed" => "\n",
        "CRLF" => "\r\n",
        "Forward slash" => "/",
        "Backslash" => "\\",
        "0x" => "0x",
        "\\x" => "\\x",
        "Nothing (separate chars)" => "",
        "None" => "",
    )
    .get(token)
    .unwrap_or(&" ")
}
