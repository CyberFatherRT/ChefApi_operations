use custom_error::custom_error;
use serde::Serialize;

custom_error! {
    #[derive(Serialize)]
    pub Error
    Error{error: String} = "{error}",
    EmptyInputError = "Input string is empty",
    OperationError{error: &'static str} = "{error}",
    InvalidInputError{error: &'static str} = "Data is not a valid string: {error}",
    MissingParameterError{error: &'static str} = "{error}",
    UnsupportedLanguageError{error: &'static str} = "Unsupported language: {error}",
    InvalidNumberOfParamsError{error: &'static str} = "{error}",
    IvalidKeyError{error: &'static str} = "Invalid key: {error}",
    InvalidParamTypeError{error: &'static str} = "{error}"
}
