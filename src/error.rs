use std::fmt;

#[derive(Debug)]
pub enum Error {
    CommandExecutionFailed(String, Option<i32>),
    IoError(std::io::Error),
}

impl Error {
    /// Get the exit code to use when this error occurs
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::CommandExecutionFailed(_, Some(code)) => *code,
            Error::CommandExecutionFailed(_, None) => 1,
            Error::IoError(_) => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CommandExecutionFailed(msg, _) => {
                write!(f, "Command execution failed: {}", msg)
            }
            Error::IoError(err) => {
                write!(f, "IO error: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
