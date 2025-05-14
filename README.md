# CSV Summary Tool

A command-line application written in Rust that parses CSV files and computes summary statistics for numeric columns.

## Overview

This tool processes CSV files and calculates key statistics (count, minimum, maximum, and average) for all numeric columns. Built with Rust and integrated with GitHub Actions for continuous integration and deployment.

## Features

- CSV file parsing using the `csv` crate
- Automatic detection of numeric columns
- Calculation of summary statistics:
  - Count of non-empty numeric values
  - Minimum value
  - Maximum value
  - Average value
- Error handling with meaningful error messages
- Integration tests
- GitHub Actions CI/CD pipeline

## Prerequisites

- Rust (stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd csv-summary-tool
```

2. Build the project:

```bash
cargo build
```

## Usage

Run the program with a CSV file as an argument:

```bash
cargo run -- path/to/your/file.csv
```

### Example

Given a CSV file `sample.csv`:

```csv
value1,value2,label
10,20,A
30,40,B
50,60,A
```

The output will be:

```bash
Summary statistics for numeric columns:
value1 -> count: 3, min: 10.00, max: 50.00, average: 30.00
value2 -> count: 3, min: 20.00, max: 60.00, average: 40.00
```

## File Format Requirements

- Standard `.csv` files
- First row must contain column names
- Supports missing values in columns

## Testing

Run the test suite:

```bash
cargo test
```

The project includes both unit tests and integration tests. Integration tests are located in `tests/integration_test.rs`.

## CI/CD Pipeline

The project uses GitHub Actions for continuous integration and deployment:

### CI Workflow

- Runs on every push to main (except tags)
- Performs:
  - Build verification
  - Test execution
  - Code linting (clippy)
  - Format checking

### CD Workflow

- Triggers on version tags (v1._ or v2._)
- Tests on multiple platforms:
  - Ubuntu
  - Windows
  - macOS
- Creates releases
- Updates version numbers
- Maintains changelog

## Error Handling

The tool implements graceful error handling:

- No panics
- Meaningful error messages to stderr
- Proper handling of edge cases

## License

This project is licensed under the MIT License - see the LICENSE file for details.
