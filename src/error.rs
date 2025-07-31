use std::fmt::{Debug, Display, Formatter};

pub(crate) struct Error {
    message: String,
}

impl Error {
    pub(crate) fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}
