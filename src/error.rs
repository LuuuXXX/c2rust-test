use std::fmt;

#[derive(Debug)]
pub enum Error {
    ConfigToolNotFound,
    CommandExecutionFailed(String, Option<i32>),
    ConfigSaveFailed(String),
    IoError(std::io::Error),
}

impl Error {
    /// Get the exit code to use when this error occurs
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::ConfigToolNotFound => 1,
            Error::CommandExecutionFailed(_, Some(code)) => *code,
            // When no exit code is available (signal termination), use 128
            // This is a common convention on Unix systems (128 + signal number)
            // but since we don't have the signal number, we use 128
            Error::CommandExecutionFailed(_, None) => 128,
            Error::ConfigSaveFailed(_) => 1,
            Error::IoError(_) => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConfigToolNotFound => {
                write!(f, "c2rust-config not found. Please install c2rust-config first.")
            }
            Error::CommandExecutionFailed(msg, _) => {
                write!(f, "Command execution failed: {}", msg)
            }
            Error::ConfigSaveFailed(msg) => {
                write!(f, "Failed to save configuration: {}", msg)
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
