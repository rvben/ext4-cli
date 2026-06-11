use assert_cmd::Command;
use predicates::prelude::*;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn explicit_json_flag_produces_valid_json() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "--output",
            "json",
            "ls",
            "/etc",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let value: serde_json::Value = serde_json::from_slice(&output).expect("must be valid JSON");
    assert!(value["items"].is_array());
    assert!(value["total"].is_number());
}

#[test]
fn explicit_text_flag_produces_text() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "--output",
            "text",
            "ls",
            "/etc",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("fstab"))
        .stdout(predicate::str::contains("{").not());
}

#[test]
fn error_envelope_is_last_line_of_stderr() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "ls",
            "/nonexistent_path_xyz",
        ])
        .assert()
        .failure()
        .get_output()
        .stderr
        .clone();

    let stderr = std::str::from_utf8(&output).unwrap();
    let last_line = stderr.lines().last().unwrap_or("");
    let value: serde_json::Value = serde_json::from_str(last_line)
        .expect("last line of stderr must be valid JSON error envelope");
    assert!(value["error"]["kind"].is_string());
    assert!(value["error"]["message"].is_string());
}

#[test]
fn error_envelope_has_known_kind() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "ls",
            "/nonexistent_path_xyz",
        ])
        .assert()
        .failure()
        .get_output()
        .stderr
        .clone();

    let stderr = std::str::from_utf8(&output).unwrap();
    let last_line = stderr.lines().last().unwrap_or("");
    let value: serde_json::Value = serde_json::from_str(last_line).unwrap();
    let kind = value["error"]["kind"].as_str().unwrap();
    let known_kinds = [
        "io_error",
        "permission_denied",
        "not_found",
        "invalid_input",
        "invalid_filesystem",
    ];
    assert!(known_kinds.contains(&kind), "unknown error kind: {kind}");
}
