# refresh

A minimal, low-level, efficient, pipe-friendly CLI tool that periodically re-executes a command or re-reads stdin, clears the screen, and prints only the latest output.

Inspired by Juniper's `refresh` command, this tool follows Unix philosophy: small, composable, and predictable.

## Features

- **Periodic Command Execution**: Run any command at specified intervals
- **Pipe Support**: Read commands from stdin for flexible composition
- **Clean Terminal Output**: Automatically clears screen between executions (TTY-aware)
- **Exit on Error**: Optional mode to stop execution on command failure
- **Logging Support**: Append output to log files with optional error-only mode
- **Signal Handling**: Clean exit on Ctrl+C (SIGINT)
- **TTY-Aware**: Automatically detects if output is redirected and adjusts behavior

## Installation

### One-Line Install (Remote)

```bash
curl -sSL https://raw.githubusercontent.com/developerdevice/refresh/main/install.sh | bash
```

### Manual Install (Local)

```bash
chmod +x install.sh
./install.sh
```

The script will detect if Rust is installed, build the project, and install it to your local path (avoiding sudo where possible).

### From Source (Manual)

```bash
git clone https://github.com/developerdevice/refresh.git
cd refresh
cargo build --release
cp target/release/refresh ~/.local/bin/ # or /usr/local/bin with sudo
```

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Syntax

```bash
refresh <interval> <command>
```

### Examples

#### 1. Command Pipe (Juniper Style)

To refresh a command using a pipe, you must pass the **command string** into `refresh`. This allows `refresh` to know what to re-execute:

```bash
echo "date" | refresh 2
```

> **Note**: Standard Unix pipes like `date | refresh 2` only pass the *output* of `date` once. To actually re-run the command periodically, use quotes: `refresh 2 "date"` or the `echo` method above.

#### 2. Basic Command Execution

Run a command every 2 seconds:

```bash
refresh 2 date
```

#### 2. Command with Arguments

Use `--` to separate the command and its arguments:

```bash
refresh 1 -- ls -lah /tmp
```

Or without `--` for simple cases:

```bash
refresh 1 ls -lah /tmp
```

#### 3. Pipe Support

Read the command from stdin:

```bash
echo "df -h" | refresh 5
```

This is useful for scripting:

```bash
cat commands.txt | refresh 3
```

#### 4. Exit on Error

Stop execution if the command fails:

```bash
refresh 1 --on-error ./health-check.sh
```

Short form:

```bash
refresh 1 -e ./health-check.sh
```

#### 5. Logging

Log all output to a file:

```bash
refresh 2 --log output.log uptime
```

Log only errors:

```bash
refresh 1 --log errors.log --log-errors-only ./monitor.sh
```

#### 6. Monitoring System Resources

Monitor disk usage:

```bash
refresh 5 df -h
```

Monitor memory:

```bash
refresh 2 free -h
```

Monitor processes:

```bash
refresh 1 -- ps aux | grep nginx
```

#### 7. Network Monitoring

Monitor network connections:

```bash
refresh 3 -- netstat -tuln
```

Ping a host:

```bash
refresh 1 -- ping -c 1 google.com
```

#### 8. File Watching

Watch file changes:

```bash
refresh 1 -- ls -lh /var/log/syslog
```

Monitor log file size:

```bash
refresh 2 -- wc -l /var/log/nginx/access.log
```

#### 9. Development Workflow

Watch test results:

```bash
refresh 2 cargo test
```

Monitor build status:

```bash
refresh 5 -- make build
```

#### 10. Combining with Other Tools

Use with `grep`:

```bash
refresh 1 -- grep ERROR /var/log/app.log
```

Use with `awk`:

```bash
refresh 2 -- ps aux | awk '{print $2, $11}'
```

Chain with pipes:

```bash
refresh 3 -- docker ps | grep running
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--on-error` | `-e` | Exit immediately if command returns non-zero exit code |
| `--log <file>` | `-l` | Append output to specified log file |
| `--log-errors-only` | | Log only when command fails (requires `--log`) |
| `--help` | `-h` | Display help information |
| `--version` | `-V` | Display version information |

## Exit Codes

- `0`: Success (normal exit)
- `1`: Command execution error or validation error
- `130`: Interrupted by SIGINT (Ctrl+C)

## Behavior

### TTY Detection

`refresh` automatically detects if stdout is a TTY:

- **TTY (terminal)**: Clears screen between executions for clean output
- **Non-TTY (pipe/redirect)**: Does not clear screen, allowing proper piping

Example with redirection:

```bash
# Screen will NOT be cleared
refresh 1 date > output.txt
```

### Signal Handling

Press `Ctrl+C` to cleanly exit the program. The tool will stop immediately and exit with code 130.

### Command Execution

Commands are executed using the system shell. Both stdout and stderr are captured and displayed.

## Comparison with Similar Tools

| Feature | refresh | watch | 
|---------|---------|-------|
| Minimal dependencies | ✅ | ❌ |
| Pipe support | ✅ | ❌ |
| Exit on error | ✅ | ✅ |
| Logging | ✅ | ❌ |
| TTY-aware | ✅ | ✅ |
| Interactive keys | ❌ | ✅ |
| Diffing | ❌ | ✅ |

## Design Philosophy

This tool strictly follows Unix philosophy:

1. **Do one thing well**: Periodically execute commands
2. **Composable**: Works seamlessly with pipes and other Unix tools
3. **Predictable**: Clear, consistent behavior
4. **Minimal**: No unnecessary features or dependencies
5. **Maintainable**: Clean, well-tested code suitable for long-term maintenance

## Development

### Building

```bash
cargo build
```

### Testing

Run all tests:

```bash
cargo test
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

### Code Quality

Format code:

```bash
cargo fmt
```

Run linter:

```bash
cargo clippy
```

Run all checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Roadmap

Future ideas (not yet implemented):

- [ ] Support for custom clear commands
- [ ] Configurable output buffering
- [ ] Timestamp prefix option for logs
- [ ] Support for multiple commands in sequence
- [ ] Configuration file support

**Note**: These are ideas only. Any new features must align with Unix philosophy and maintain simplicity.

## Contributing

Contributions are welcome! Please ensure:

1. Code passes `cargo fmt`, `cargo clippy`, and `cargo test`
2. New features include tests
3. Changes align with Unix philosophy
4. Documentation is updated

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Inspired by Juniper's `refresh` command and the Unix philosophy of simple, composable tools.
