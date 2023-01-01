use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum AppError {
    Internal(ErrorType, Option<String>),
    Code(hyper::StatusCode),
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum ErrorType {
    #[default]
    GeneralTechnicalError,
}

#[derive(Debug, Serialize)]
struct ErrorWrapper {
    error: ErrorType,
    description: Option<String>,
}

impl<E: std::fmt::Display + Debug> From<E> for AppError {
    #[inline]
    #[track_caller]
    fn from(err: E) -> Self {
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        let caller_file = caller_location.file();

        let msg = format!("{caller_file}:{caller_line_number}: {err}");
        AppError::Internal(ErrorType::default(), Some(msg))
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal(error, description) => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&ErrorWrapper { error, description }).unwrap(),
            ),
            Self::Code(code) => (code, String::from("")),
        }
        .into_response()
    }
}
