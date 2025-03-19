use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Argument(String),
    Value(String),
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(value: &'a str) -> Self {
        Self::Custom(value.to_string())
    }
}

impl std::error::Error for Error {}
