use clap::Parser;

/// A minimal, pipe-friendly CLI tool that periodically re-executes a command
#[derive(Parser, Debug)]
#[command(name = "refresh")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Interval in seconds between command executions
    #[arg(value_name = "INTERVAL")]
    pub interval: u64,

    /// Command to execute (use -- to separate command with arguments)
    #[arg(value_name = "COMMAND", num_args = 0..)]
    pub command: Vec<String>,

    /// Exit immediately if command returns non-zero exit code
    #[arg(short = 'e', long = "on-error")]
    pub exit_on_error: bool,

    /// Log output to file (appends on each refresh)
    #[arg(short = 'l', long = "log", value_name = "FILE")]
    pub log_file: Option<String>,

    /// Log only errors (requires --log)
    #[arg(long = "log-errors-only", requires = "log_file")]
    pub log_errors_only: bool,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Cli::parse()
    }

    /// Validate the parsed arguments
    pub fn validate(&self) -> crate::errors::Result<()> {
        if self.interval == 0 {
            return Err(crate::errors::RefreshError::CliParse(
                "Interval must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the command as a single string for display purposes
    #[allow(dead_code)]
    pub fn command_string(&self) -> String {
        self.command.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_zero_interval() {
        let cli = Cli {
            interval: 0,
            command: vec!["echo".to_string(), "test".to_string()],
            exit_on_error: false,
            log_file: None,
            log_errors_only: false,
        };

        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_validate_valid_interval() {
        let cli = Cli {
            interval: 1,
            command: vec!["echo".to_string(), "test".to_string()],
            exit_on_error: false,
            log_file: None,
            log_errors_only: false,
        };

        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_command_string() {
        let cli = Cli {
            interval: 1,
            command: vec!["echo".to_string(), "hello".to_string(), "world".to_string()],
            exit_on_error: false,
            log_file: None,
            log_errors_only: false,
        };

        assert_eq!(cli.command_string(), "echo hello world");
    }
}
