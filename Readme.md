# NTP-Synchronized Clock

A robust Rust-based NTP-synchronized clock application with advanced features for time synchronization and display.

## Features

### Core Functionality
- **NTP Synchronization**: Automatically fetches time from multiple NTP servers (Google, Cloudflare, pool.ntp.org)
- **Periodic Updates**: Background thread periodically updates time (configurable interval)
- **Drift Correction**: Automatically detects and corrects time drift
- **Fallback Mechanism**: Falls back to default time if NTP servers are unreachable

### Configuration Options
- **Custom NTP Servers**: Specify your own NTP servers via command-line
- **Configurable Update Interval**: Set how often to sync with NTP servers
- **Configurable Display Interval**: Set how often to display the current time
- **Timezone Support**: Display time in different timezones using UTC offset
- **Statistics**: Track sync success rate and attempt counts

### Quality Features
- **Comprehensive Logging**: Built-in logging with configurable verbosity
- **Graceful Shutdown**: Proper cleanup with Ctrl+C handler
- **Error Handling**: Robust error handling throughout the codebase
- **Unit Tests**: Comprehensive test coverage for core functionality
- **Documentation**: Full API documentation for all public functions

## Installation

```bash
cargo build --release
```

## Usage

### Basic Usage
```bash
# Run with default settings (updates every 10s, displays every 1s)
cargo run

# Show help
cargo run -- --help
```

### Advanced Usage

```bash
# Custom update and display intervals
cargo run -- --interval 30 --display-interval 2

# Custom NTP servers
cargo run -- --server time.nist.gov:123 --server time.windows.com:123

# Display time in different timezone (e.g., EST = UTC-5)
cargo run -- --timezone-offset -5

# Show synchronization statistics
cargo run -- --show-stats

# Enable verbose logging for debugging
cargo run -- --verbose

# Combine multiple options
cargo run -- --interval 60 --timezone-offset -5 --show-stats --verbose
```

## Command-Line Options

- `-i, --interval <INTERVAL>`: NTP update interval in seconds (default: 10)
- `-d, --display-interval <DISPLAY_INTERVAL>`: Display interval in seconds (default: 1)
- `-s, --server <SERVER>`: Custom NTP server (can be specified multiple times)
- `-t, --timezone-offset <TIMEZONE_OFFSET>`: Timezone offset in hours (default: 0 for UTC)
- `-v, --verbose`: Enable verbose logging for debugging
- `--show-stats`: Show synchronization statistics (attempts, success rate)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Example Output

### Standard Output
```
Time (UTC+0): 2026-02-03 06:50:57
Time (UTC+0): 2026-02-03 06:50:58
Time (UTC+0): 2026-02-03 06:50:59
```

### With Statistics
```
Time (UTC-5): 2026-02-03 01:50:57 | Syncs: 5/5 (100.0% success)
Time (UTC-5): 2026-02-03 01:50:58 | Syncs: 5/5 (100.0% success)
```

## Architecture

The project is structured with a clean separation between library and binary:

- **src/lib.rs**: Core clock functionality, NTP client, and statistics tracking
- **src/main.rs**: Command-line interface and application entry point
- **tests/**: Integration tests for the library

### Key Components

- **Clock**: Main struct managing NTP synchronization and time tracking
- **SyncStats**: Statistics tracking for sync attempts and success rate
- **NTP Client**: Handles communication with NTP servers using UDP
- **Background Thread**: Periodically updates time from NTP servers

## Technical Details

### NTP Protocol
- Uses NTP version 3 protocol
- 3-second timeout for network operations
- Automatic server failover if one server is unavailable
- Timestamp extraction from NTP response packets

### Time Management
- Tracks elapsed time using monotonic clock (`std::time::Instant`)
- Calculates current time as: `last_sync_time + elapsed`
- Drift correction threshold: 100ms
- Falls back to January 1, 2000 if all NTP servers fail

## Development

### Running Tests
```bash
cargo test
```

### Building Release Version
```bash
cargo build --release
./target/release/clock --help
```

### Code Structure
```
Clock-NTP/
├── Cargo.toml          # Project configuration
├── Readme.md           # This file
├── src/
│   ├── lib.rs         # Library code
│   └── main.rs        # Binary code
└── tests/
    └── integration_test.rs  # Integration tests
```

## Dependencies

- **chrono**: Date and time handling
- **clap**: Command-line argument parsing
- **log**: Logging facade
- **env_logger**: Logger implementation
- **ctrlc**: Signal handling for graceful shutdown

## Future Enhancements

Potential features for future versions:
- Configuration file support (TOML/YAML)
- Multiple time format outputs (RFC3339, Unix timestamp, etc.)
- Web UI or REST API for monitoring
- Daemon/service mode for background operation
- Enhanced drift correction algorithms
- Support for NTPv4 and higher precision time protocols

## License

This project is provided as-is for educational and practical use.

## Contributing

Contributions are welcome! Please ensure:
- All tests pass (`cargo test`)
- Code is properly formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)
- Documentation is updated for new features
