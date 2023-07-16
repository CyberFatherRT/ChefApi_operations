use super::error::EmptyInputString::EmptyInputString;
use super::Operation;

fn A1Z26CipherDecoder(request: Operation) -> Result<String, EmptyInputString> {

    let separator = request.params.get(0).unwrap();
    let cipher_text = request.input;

    if cipher_text.is_empty() {
        return Err(EmptyInputString);
    }

    let plain_text = String::new();

    for letter in cipher_text.split(separator) {

    }
    todo!()
}