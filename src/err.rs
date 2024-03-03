use std::error::Error;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Results returned by the `shadow-rs` build process.
/// For more information see [`ShadowError`].
pub type SdResult<T> = Result<T, ShadowError>;

/// `shadow-rs` build process errors.
/// This type wraps multiple kinds of underlying errors that can occur downstream of `shadow-rs`, such as [`std::io::Error`].
#[derive(Debug)]
pub enum ShadowError {
    String(String),
}

impl ShadowError {
    pub fn new(err: impl Error) -> Self {
        ShadowError::String(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ShadowError {
    fn from(e: FromUtf8Error) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl From<std::io::Error> for ShadowError {
    fn from(e: std::io::Error) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl From<String> for ShadowError {
    fn from(e: String) -> Self {
        ShadowError::String(e)
    }
}

impl From<&str> for ShadowError {
    fn from(e: &str) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl From<std::env::VarError> for ShadowError {
    fn from(e: std::env::VarError) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl From<std::num::ParseIntError> for ShadowError {
    fn from(e: std::num::ParseIntError) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl Display for ShadowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShadowError::String(err) => f.write_str(err),
        }
    }
}

impl StdError for ShadowError {}
