use assert_cmd::Command;
use predicates::prelude::*;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn cat_file_outputs_content() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "cat", "/etc/fstab"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ext4"));
}

#[test]
fn cat_opaque_output_is_not_reformatted_as_json() {
    let source = fixture("rich.img");
    let default = Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &source, "cat", "/etc/fstab"])
        .output()
        .unwrap();
    let explicit_json = Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &source, "--output", "json", "cat", "/etc/fstab"])
        .output()
        .unwrap();

    assert!(default.status.success());
    assert!(explicit_json.status.success());
    assert_eq!(explicit_json.stdout, default.stdout);
}

#[test]
fn cat_nonexistent_exits_3() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "cat", "/no/such/file"])
        .assert()
        .failure()
        .code(3);
}

#[test]
fn cat_directory_fails() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "cat", "/etc"])
        .assert()
        .failure();
}
