# c2rust-test

C project test execution tool for c2rust workflow.

## Overview

`c2rust-test` is a command-line tool that executes test commands for C build projects and automatically saves the configuration using `c2rust-config`. This tool is part of the c2rust workflow for managing C to Rust translations.

## Installation

### From Source

```bash
cargo install --path .
```

Or build locally:

```bash
cargo build --release
# Binary will be in target/release/c2rust-test
```

## Prerequisites

This tool requires `c2rust-config` to be installed. Install it from:
https://github.com/LuuuXXX/c2rust-config

### Environment Variables

- `C2RUST_CONFIG`: Optional. Path to the c2rust-config binary. If not set, the tool will look for `c2rust-config` in your PATH.

## Usage

### Basic Command

```bash
c2rust-test test --dir <directory> -- <test-command> [args...]
```

The `test` subcommand will:
1. Execute the specified test command in the specified directory
2. Save the test configuration to c2rust-config for later use

### Examples

#### Running Make Tests

```bash
c2rust-test test --dir /path/to/project -- make test
```

#### Running Custom Test Script

```bash
c2rust-test test --dir . -- ./run_tests.sh
```

#### Running Tests with CMake

```bash
c2rust-test test --dir build -- ctest --output-on-failure
```

#### Running Tests with Feature Flag

You can specify a feature name to organize different test configurations:

```bash
c2rust-test test --feature debug --dir /path/to/project -- make test
```

#### Using Custom c2rust-config Path

If `c2rust-config` is not in your PATH or you want to use a specific version:

```bash
export C2RUST_CONFIG=/path/to/custom/c2rust-config
c2rust-test test --dir /path/to/project -- make test
```

### Command Line Options

- `--dir <directory>`: Directory to execute test command (required)
- `--feature <name>`: Optional feature name for the configuration (default: "default")
- `--`: Separator between c2rust-test options and the test command
- `<command> [args...]`: The test command and its arguments to execute

### Help

Get general help:

```bash
c2rust-test --help
```

Get help for the test subcommand:

```bash
c2rust-test test --help
```

## How It Works

1. **Validation**: Checks if `c2rust-config` is installed
2. **Execution**: Runs the specified test command in the specified directory
3. **Configuration**: Saves two configuration values:
   - `test.dir`: The directory where tests are executed
   - `test`: The full test command string

## Configuration Storage

The tool uses `c2rust-config` to store test configurations. These can be retrieved later by other c2rust tools.

Example stored configuration:
```
test.dir = "/path/to/project"
test = "make test"
```

With a feature:
```
test.dir = "/path/to/project" (for feature "debug")
test = "make test" (for feature "debug")
```

## Error Handling

The tool will exit with an error if:
- `c2rust-config` is not found in PATH
- The test command fails to execute
- The configuration cannot be saved

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

Note: Some integration tests may fail if `c2rust-config` is not installed.

### Running Unit Tests Only

```bash
cargo test --lib
```

## License

This project is part of the c2rust ecosystem.

## Related Projects

- [c2rust-config](https://github.com/LuuuXXX/c2rust-config) - Configuration management tool
- [c2rust-clean](https://github.com/LuuuXXX/c2rust-clean) - Build artifact cleaning tool

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.