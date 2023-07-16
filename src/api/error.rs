use custom_error::custom_error;

custom_error! { pub Error
    EmptyInputError = "Input string is empty",
    OperationError{error: &'static str} = "{error}",
    InvalidInputError{error: &'static str} = "Invalid input: {error}"
}
