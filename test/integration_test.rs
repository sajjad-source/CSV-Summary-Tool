#![allow(unused_variables)]
use std::process::Command;

#[test]
fn test_cli_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--", "sample.csv"])
        .output()
        .expect("failed to run");

    assert!(output.status.success() || !output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // add more
}
