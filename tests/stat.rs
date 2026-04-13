use assert_cmd::Command;
use predicates::prelude::*;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn stat_file_succeeds() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "stat", "/etc/fstab"])
        .assert()
        .success()
        .stdout(predicate::str::contains("type:"))
        .stdout(predicate::str::contains("size:"))
        .stdout(predicate::str::contains("mode:"));
}

#[test]
fn stat_directory_succeeds() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "stat", "/etc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("directory"));
}

#[test]
fn stat_json_output() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "stat", "--json", "/etc/fstab"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["type"], "file");
    assert!(json["size"].as_u64().unwrap() > 0);
    assert!(json.get("mode").is_some());
    assert!(json.get("uid").is_some());
    assert!(json.get("gid").is_some());
}

#[test]
fn stat_nonexistent_exits_3() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "stat", "/no/such/path"])
        .assert()
        .failure()
        .code(3);
}
