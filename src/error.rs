use std::fmt;

/// Errors that happen during processing.
#[derive(Debug)]
pub enum Error {
    /// Error writing output.
    OutputWrite(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputWrite(_) => write!(f, "Error writing output"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OutputWrite(error) => Some(error),
        }
    }
}
