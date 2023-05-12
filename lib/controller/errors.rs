use std::fmt;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

#[derive(Debug)]
pub enum ErrorKind {
    CollectionNotFound,
    ObjectNotFound,
    Serialization(serde_json::Error),
    Unexpected(Box<dyn std::error::Error>),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            ErrorKind::CollectionNotFound => write!(f, "colelction could not be found"),
            ErrorKind::ObjectNotFound => write!(f, "object could not be found"),
            ErrorKind::Serialization(err) => write!(f, "object serialization failed: {err}"),
            ErrorKind::Unexpected(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error(Box::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        ErrorKind::Serialization(value).into()
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        ErrorKind::Unexpected(value.into()).into()
    }
}
