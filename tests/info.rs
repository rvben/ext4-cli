use assert_cmd::Command;
use predicates::prelude::*;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn info_minimal_succeeds() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("minimal.img"), "info"])
        .assert()
        .success();
}

#[test]
fn source_missing_gives_error() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["ls", "/"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no source specified"));
}

#[test]
fn source_nonexistent_gives_error() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", "/nonexistent/path.img", "info"])
        .assert()
        .failure()
        .code(1);
}
