use custom_error::custom_error;

custom_error! { pub Error
    EmptyInputError = "Input string is empty",
    OperationError{error: &'static str} = "{error}",
    InvalidInputError{error: &'static str} = "Data is not a valid string: {error}",
    MissingParameterError = "Missing parameter",
    UnsupportedLanguageError = "Unsupported language",
    InvalidNumberOfParamsError = "Invalid number of parameters",
    IvalidKeyError = "Invalid key",
}
