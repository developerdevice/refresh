use std::fmt;

/// Custom error type for the refresh application
#[derive(Debug)]
pub enum RefreshError {
    /// Error parsing command-line arguments
    CliParse(String),
    /// Error executing the command
    CommandExecution(String),
    /// Error with I/O operations (file, stdin, stdout)
    Io(std::io::Error),
    /// Error with signal handling
    Signal(String),
    /// Error with logging
    Logging(String),
}

impl fmt::Display for RefreshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefreshError::CliParse(msg) => write!(f, "CLI error: {}", msg),
            RefreshError::CommandExecution(msg) => write!(f, "Execution error: {}", msg),
            RefreshError::Io(err) => write!(f, "I/O error: {}", err),
            RefreshError::Signal(msg) => write!(f, "Signal handling error: {}", msg),
            RefreshError::Logging(msg) => write!(f, "Logging error: {}", msg),
        }
    }
}

impl std::error::Error for RefreshError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RefreshError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RefreshError {
    fn from(err: std::io::Error) -> Self {
        RefreshError::Io(err)
    }
}

/// Result type alias for refresh operations
pub type Result<T> = std::result::Result<T, RefreshError>;
