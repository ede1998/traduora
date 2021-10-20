use std::any;
use std::error::Error;

use thiserror::Error;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to serialize to JSON: {}", source)]
    SerdeJson {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create request body: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// JSON deserialization from Traduora failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Traduora returned an error message.
    #[error("traduora server error: {}", msg)]
    Traduora {
        /// The error message from Traduora.
        msg: String,
    },
    /// Traduora returned an error without JSON information.
    #[error("traduora internal server error {}", status)]
    TraduoraService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The error data from Traduora.
        data: Vec<u8>,
    },
    /// Traduora returned an error object.
    #[error("traduora server error: {:?}", obj)]
    TraduoraObject {
        /// The error object from Traduora.
        obj: serde_json::Value,
    },
    /// Traduora returned an HTTP error with JSON we did not recognize.
    #[error("traduora server error: {:?}", obj)]
    TraduoraUnrecognized {
        /// The full object from Traduora.
        obj: serde_json::Value,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        Self::Client { source }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::TraduoraService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }

    pub(crate) fn from_traduora(value: serde_json::Value) -> Self {
        let error_value = value
            .pointer("/message")
            .or_else(|| value.pointer("/error"));

        match error_value {
            Some(error_value) => match error_value.as_str() {
                Some(msg) => Self::Traduora { msg: msg.into() },
                None => Self::TraduoraObject {
                    obj: error_value.clone(),
                },
            },
            None => Self::TraduoraUnrecognized { obj: value },
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use thiserror::Error;

    use crate::ApiError;

    #[derive(Debug, Error)]
    #[error("my error")]
    enum MyError {}

    #[test]
    fn traduora_error_error() {
        let obj = json!({
            "error": "error contents",
        });

        let err: ApiError<MyError> = ApiError::from_traduora(obj);
        if let ApiError::Traduora { msg } = err {
            assert_eq!(msg, "error contents");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn traduora_error_message_string() {
        let obj = json!({
            "message": "error contents",
        });

        let err: ApiError<MyError> = ApiError::from_traduora(obj);
        if let ApiError::Traduora { msg } = err {
            assert_eq!(msg, "error contents");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn traduora_error_message_object() {
        let err_obj = json!({
            "blah": "foo",
        });
        let obj = json!({
            "message": err_obj,
        });

        let err: ApiError<MyError> = ApiError::from_traduora(obj);
        if let ApiError::TraduoraObject { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn traduora_error_message_unrecognized() {
        let err_obj = json!({
            "some_weird_key": "an even weirder value",
        });

        let err: ApiError<MyError> = ApiError::from_traduora(err_obj.clone());
        if let ApiError::TraduoraUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
