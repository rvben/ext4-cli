use assert_cmd::Command;
use predicates::prelude::*;
use serde_json;

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

#[test]
fn info_minimal_human_output() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("minimal.img"), "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("label:"))
        .stdout(predicate::str::contains("block_size:"));
}

#[test]
fn info_minimal_json_output() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("minimal.img"), "info", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(json.get("uuid").is_some());
    assert!(json.get("label").is_some());
    assert!(json.get("block_size").is_some());
    assert!(json.get("inodes_count").is_some());
}

#[test]
fn info_rich_label() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rich"));
}
