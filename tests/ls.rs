use assert_cmd::Command;
use predicates::prelude::*;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn ls_root_succeeds() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("minimal.img"), "ls"])
        .assert()
        .success();
}

#[test]
fn ls_defaults_to_root() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains("etc"))
        .stdout(predicate::str::contains("home"));
}

#[test]
fn ls_subdir() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "ls", "/etc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fstab"))
        .stdout(predicate::str::contains("passwd"));
}

#[test]
fn ls_long_shows_mode_and_size() {
    // Explicit --output text because piped stdout causes auto-JSON mode.
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "--output",
            "text",
            "ls",
            "-l",
            "/etc",
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"-rw.* fstab").unwrap());
}

#[test]
fn ls_json_output_is_envelope() {
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

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(json["items"].is_array());
    assert!(json["total"].is_number());
    let arr = json["items"].as_array().unwrap();
    let names: Vec<&str> = arr.iter().map(|e| e["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"fstab"));
    assert!(names.contains(&"passwd"));
}

#[test]
fn ls_nonexistent_path_exits_3() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "ls", "/nonexistent"])
        .assert()
        .failure()
        .code(3);
}

#[test]
fn ls_on_file_exits_with_error() {
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "ls", "/etc/fstab"])
        .assert()
        .failure();
}

#[test]
fn ls_hides_dotfiles_by_default() {
    // `..` is present in every ext4 directory but must be hidden without --all
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "ls", "/etc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("..").not());
}

#[test]
fn ls_all_shows_dotfiles() {
    // `..` must appear when --all is given
    Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "ls", "--all", "/etc"])
        .assert()
        .success()
        .stdout(predicate::str::contains(".."));
}
