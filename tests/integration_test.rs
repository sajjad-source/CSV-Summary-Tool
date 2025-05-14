use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[test]
fn summary_contains_expected_columns() {
    let mut exe = Command::cargo_bin("rust_csv_cli").unwrap();
    let sample = Path::new("sample.csv");
    assert!(sample.exists(), "sample.csv missing");

    exe.arg(sample)
        .assert()
        .success()
        .stdout(predicate::str::contains("value1"))
        .stdout(predicate::str::contains("value2"))
        .stdout(predicate::str::contains("average"));
}

#[test]
fn handles_missing_numeric_values() {
    // create a tiny CSV *with* a blank numeric cell
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "n1,n2\n10,20\n,40\n30,\n").unwrap();

    let mut cmd = Command::cargo_bin("rust_csv_cli").unwrap();
    cmd.arg(tmp.path())
        .assert()
        .success()
        // numeric column n1 has 2 non‑empty values -> count: 2
        .stdout(predicate::str::contains("n1").and(predicate::str::contains("count:   2")))
        // numeric column n2 has 2 non‑empty values -> count: 2
        .stdout(predicate::str::contains("n2").and(predicate::str::contains("count:   2")));
}
