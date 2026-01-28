use std::fs::OpenOptions;
use std::io::{self, BufRead, IsTerminal, Write};

use crate::errors::{RefreshError, Result};

/// Check if stdout is a TTY
pub fn is_tty() -> bool {
    io::stdout().is_terminal()
}

/// Check if stdin is a TTY
pub fn is_stdin_tty() -> bool {
    io::stdin().is_terminal()
}

/// Clear the terminal screen
pub fn clear_screen() -> Result<()> {
    // Use ANSI escape sequence to clear screen
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

/// Read command from stdin (for pipe support)
pub fn read_command_from_stdin() -> Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    handle
        .read_line(&mut line)
        .map_err(RefreshError::Io)?;

    Ok(line.trim().to_string())
}

/// Logger for writing command output to a file
pub struct Logger {
    file_path: String,
    log_errors_only: bool,
}

impl Logger {
    /// Create a new logger
    pub fn new(file_path: String, log_errors_only: bool) -> Self {
        Logger {
            file_path,
            log_errors_only,
        }
    }

    /// Log output to file
    pub fn log(&self, output: &str, is_error: bool) -> Result<()> {
        // Skip logging if we only want errors and this is not an error
        if self.log_errors_only && !is_error {
            return Ok(());
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| RefreshError::Logging(format!("Failed to open log file: {}", e)))?;

        writeln!(file, "{}", output)
            .map_err(|e| RefreshError::Logging(format!("Failed to write to log file: {}", e)))?;
        file.flush()
            .map_err(|e| RefreshError::Logging(format!("Failed to flush log file: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_logger_writes_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let logger = Logger::new(path.clone(), false);
        logger.log("test output", false).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("test output"));
    }

    #[test]
    fn test_logger_errors_only() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let logger = Logger::new(path.clone(), true);

        // Should not log non-errors
        logger.log("normal output", false).unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(!content.contains("normal output"));

        // Should log errors
        logger.log("error output", true).unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("error output"));
    }

    #[test]
    fn test_logger_appends() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let logger = Logger::new(path.clone(), false);
        logger.log("first", false).unwrap();
        logger.log("second", false).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("first"));
        assert!(content.contains("second"));
    }
}
