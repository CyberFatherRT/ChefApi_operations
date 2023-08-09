use custom_error::custom_error;
use serde::{Deserialize, Serialize};

custom_error! {
    #[derive(Deserialize, Serialize)]
    pub Error
        Error{error: String} = "{error}",
        EmptyInputError = "Input string is empty",
        OperationError{error: String} = "{error}",
        InvalidInputError{error: String} = "Data is not a valid string: {error}",
        MissingParameterError{error: String} = "{error}",
        UnsupportedLanguageError{error: String} = "Unsupported language: {error}",
        InvalidNumberOfParamsError{error: String} = "{error}",
        IvalidKeyError{error: String} = "Invalid key: {error}",
        InvalidParamTypeError{error: String} = "{error}"
}
