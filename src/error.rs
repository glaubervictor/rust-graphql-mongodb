use juniper::{ScalarValue, IntoFieldError, FieldError, graphql_value};

pub enum CustomError {
    #[warn(dead_code)]
    WhateverNotSet,
}

impl<S: ScalarValue> IntoFieldError<S> for CustomError {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            CustomError::WhateverNotSet => FieldError::new(
                "Whatever does not exist",
                graphql_value!({
                    "type": "NO_WHATEVER"
                }),
            ),
        }
    }
}
