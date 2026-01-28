mod cli;
mod errors;
mod io;
mod runner;
mod signal;

use std::process;

use cli::Cli;
use errors::Result;
use io::{read_command_from_stdin, Logger};
use runner::RunnerConfig;
use signal::setup_signal_handler;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // Parse and validate CLI arguments
    let cli = Cli::parse_args();
    cli.validate()?;

    // Setup signal handler
    let interrupted = setup_signal_handler()?;

    // Determine the command to execute
    let command = if cli.command.is_empty() {
        if io::is_stdin_tty() {
            return Err(errors::RefreshError::CliParse(
                "No command provided. Usage: refresh <interval> <command> OR command | refresh <interval>".to_string(),
            ));
        }
        
        // Read from stdin if no command provided
        let cmd_string = read_command_from_stdin()?;
        if cmd_string.is_empty() {
            return Err(errors::RefreshError::CliParse(
                "No command provided via stdin".to_string(),
            ));
        }

        // Split the command string into parts
        shell_words::split(&cmd_string).map_err(|e| {
            errors::RefreshError::CliParse(format!("Failed to parse command from stdin: {}", e))
        })?
    } else {
        cli.command
    };

    // Setup logger if requested
    let logger = cli
        .log_file
        .map(|path| Logger::new(path, cli.log_errors_only));

    // Create runner configuration
    let config = RunnerConfig::new(command, cli.interval, cli.exit_on_error, logger);

    // Run the command periodically
    let exit_code = runner::run(config, interrupted)?;

    process::exit(exit_code);
}
