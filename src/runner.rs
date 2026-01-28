use std::process::{Command, Stdio};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::errors::{RefreshError, Result};
use crate::io::{clear_screen, is_tty, Logger};
use crate::signal::is_interrupted;

/// Configuration for the command runner
pub struct RunnerConfig {
    pub command: Vec<String>,
    pub interval: u64,
    pub exit_on_error: bool,
    pub logger: Option<Logger>,
    pub should_clear: bool,
}

impl RunnerConfig {
    /// Create a new runner configuration
    pub fn new(
        command: Vec<String>,
        interval: u64,
        exit_on_error: bool,
        logger: Option<Logger>,
    ) -> Self {
        RunnerConfig {
            command,
            interval,
            exit_on_error,
            logger,
            should_clear: is_tty(),
        }
    }
}

/// Execute a single command and return its output and exit status
fn execute_command(command: &[String]) -> Result<(String, bool)> {
    if command.is_empty() {
        return Err(RefreshError::CommandExecution(
            "No command provided".to_string(),
        ));
    }

    let program = &command[0];
    let args = &command[1..];

    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            RefreshError::CommandExecution(format!("Failed to execute '{}': {}", program, e))
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let combined_output = if stderr.is_empty() {
        stdout
    } else {
        format!("{}{}", stdout, stderr)
    };

    let success = output.status.success();

    Ok((combined_output, success))
}

/// Run the command periodically until interrupted
pub fn run(config: RunnerConfig, interrupted: Arc<AtomicBool>) -> Result<i32> {
    loop {
        // Check if interrupted before executing
        if is_interrupted(&interrupted) {
            return Ok(130); // Standard exit code for SIGINT
        }

        // Clear screen if we're in a TTY
        if config.should_clear {
            clear_screen()?;
        }

        // Execute the command
        let (output, success) = execute_command(&config.command)?;

        // Print output to stdout
        print!("{}", output);
        std::io::Write::flush(&mut std::io::stdout())?;

        // Log if logger is configured
        if let Some(ref logger) = config.logger {
            logger.log(&output, !success)?;
        }

        // Handle exit-on-error
        if config.exit_on_error && !success {
            return Ok(1);
        }

        // Sleep for the interval, checking for interrupts
        let sleep_duration = Duration::from_secs(config.interval);
        let check_interval = Duration::from_millis(100);
        let mut elapsed = Duration::ZERO;

        while elapsed < sleep_duration {
            if is_interrupted(&interrupted) {
                return Ok(130);
            }

            let sleep_time = std::cmp::min(check_interval, sleep_duration - elapsed);
            thread::sleep(sleep_time);
            elapsed += sleep_time;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command_success() {
        let command = vec!["echo".to_string(), "test".to_string()];
        let result = execute_command(&command);

        assert!(result.is_ok());
        let (output, success) = result.unwrap();
        assert!(success);
        assert!(output.contains("test"));
    }

    #[test]
    fn test_execute_command_failure() {
        let command = vec!["false".to_string()];
        let result = execute_command(&command);

        assert!(result.is_ok());
        let (_output, success) = result.unwrap();
        assert!(!success);
    }

    #[test]
    fn test_execute_command_empty() {
        let command: Vec<String> = vec![];
        let result = execute_command(&command);

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_command_not_found() {
        let command = vec!["nonexistent_command_xyz123".to_string()];
        let result = execute_command(&command);

        assert!(result.is_err());
    }

    #[test]
    fn test_runner_config_new() {
        let command = vec!["echo".to_string(), "test".to_string()];
        let config = RunnerConfig::new(command.clone(), 1, false, None);

        assert_eq!(config.command, command);
        assert_eq!(config.interval, 1);
        assert!(!config.exit_on_error);
        assert!(config.logger.is_none());
    }
}
