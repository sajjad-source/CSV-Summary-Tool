use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

#[test]
fn summary_contains_expected_columns() {
    let exe = Command::cargo_bin("csv_summary_tool").unwrap();
    let sample = Path::new("sample.csv");
    assert!(sample.exists(), "sample.csv missing");

    exe.arg(sample)
        .assert()
        .success()
        .stdout(predicate::str::contains("value1"))
        .stdout(predicate::str::contains("value2"))
        .stdout(predicate::str::contains("average"));
}
