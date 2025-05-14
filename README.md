# :1234: CSV Summary Tool 

#### Using Rust and GitHub Actions

## Overview

In this assignment, you will develop a command-line application in **Rust** that parses a CSV (Comma-Separated Values) file and computes summary statistics such as **count**, **minimum**, **maximum**, and **average** for each numeric column. The tool should run locally your laptop, accept a filename as the only argument, and return structured output.

This assignment gives you hands-on experience with:

- Parsing structured data formats
- Implementing command-line interfaces in Rust
- Performing numerical operations and error handling
- Writing and running automated tests
- Using GitHub Actions for CI (build, test, lint, format)


---

## Learning Objectives

By completing this assignment, you will be able to:

- ✅ Read and parse CSV files using Rust and the `csv` crate
- ✅ Compute aggregate statistics from data
- ✅ Gracefully handle input errors and edge cases
- ✅ Write integration tests
- ✅ Set up continuous integration with GitHub Actions

---

## Functional Requirements

Your program must:

- Accept a single argument that is a path to a CSV file
- Read and parse the contents using the `csv` crate
- Identify all numeric columns (f64-parsable)
- Compute and print, for each numeric column:
  - Count of non-empty numeric values
  - Minimum value
  - Maximum value
  - Average value
- Print the results in a human-readable format to `stdout`

---

## Example Usage

Given the file `sample.csv`:

```csv
value1,value2,label
10,20,A
30,40,B
50,60,A
```

Then running your program with

```bash
cargo run -- sample.csv
```

the output produced would be

```bash
Summary statistics for numeric columns:
value1 -> count: 3, min: 10.00, max: 50.00, average: 30.00
value2 -> count: 3, min: 20.00, max: 60.00, average: 40.00
```

Note that `value1` and `value2` are the header columns for columns with numeric data.

## File format assumptions

- Standard `.csv` files
- The first row contains column names
- Columns may have missing values (see `sample-missing-data.csv`, line 2).

---

## Tasks

1. Copy the provided GitHub template repository

## Errors

Prevent panics !  Produce meaningful error messages to `stderr`.

## Testing

In addition to any unit-tests you have in your `.rs` source code, create an `integration_test.rs` file under the `tests/` directory of your crate. That file should 

1. Create a test `.csv` file
2. Run your binary and capture the output to verify expected values appear. Here's how to do this:

```rust
#![allow(unused_imports)]
use std::process::Command;  // needed when running test_summary_output

// main code

#[test]
fn test_summary_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "sample.csv"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("value1"));
    assert!(stdout.contains("average"));
}
```

You can run this test manually with `cargo test`

## GitHub Actions CI

Setup two GitHub Actions workflow files at the top of your directory:
Two workflows: CI and CD

#### CI should do what we usually have, with the added "tags-ignore" since we want the other workflow to run when tags are pushed.

  - ```yaml
    on:
      push:
        branches:
          - 'main'
        tags-ignore:
    ```

It should perform the usual steps we've seen:

1. Checkout repository
1. Install Rust (stable)
1. `cargo build --verbose`
1. `cargo test --verbose`
1. `cargo clippy --all-targets --all-features -- -D warnings`
1. `cargo fmt --all -- --check`

#### CD is only for new tags of the form "v1.**" or "v2.**"

  - ```yaml
    on:
      push:
        tags:
          - v1.*
          - v2.*
    ```

The CD workflow should:

- run on Ubuntu

- set ENV vars

  - CRATE_PATHS=.

- have these steps:

  - `check-version`

    - Check version to release in `Cargo.toml` files using `get-version.sh` and save in `outputs.version` 
    - Append a line to `CHANGELOG.md`: "CD started on version `outputs.version`"

  - test-ubuntu

    - `needs: check-version`
    - Test on Ubuntu (latest)

  - test-windows

    - `needs: check-version`
    - Test on Windows (latest)

  - test-macos

    - `needs: check-version`
    - Test on MacOS

  - create-release:

    - ```yml
      needs:
        - test-ubuntu
        - test-windows
        - test-macos
      ```

    - increment the version number in `Cargo.toml` using `increment-version.sh`

    - fetch the new version number back into `outputs.version`

    - Append a line with the latest version and its release date in the `CHANGELOG.md` file and push the changes

  - Tag the created commit with the name of the released version and push the changes.

You will have to find a way to get the version info obtained in `step: check-version` to use as 
the parameter to the `increment-version.sh` to update it in `Cargo.toml`.


## Secrets

to do the CD you will need a GitHub Personal Access Token. On GitHub, click on your icon in top-right corner, then settings->Developer Settings->Personal access tokens -> Tokens (classic) then Generate New Token.

You have to save that token in an Environment for your repository. Go to Settings-> Environments and add an Environment named "DEPLOYMENT" and a secret there named GIT-TOKEN with the value of the access token.

In the CD workflow you can extract that secret by setting an ENV variable:

```yml
        env:
          GITHUB_TOKEN: ${{ secrets.GIT_TOKEN }}

```

## Grading rubric (25 points)

| Feature                                        | Points |
| ---------------------------------------------- | -----: |
| Reads and parses CSV file correctly            |      5 |
| Correct computation of summary statistics      |      3 |
| Integration tests with `cargo test`            |      5 |
| Workflows: CI (4) and CD (4)                   |      8 |
| Graceful error handling (especially no panics) |      4 |

## License

This project is provided for educational purposes and licensed under the MIT License.
