use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn cp_file_to_directory() {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source", &fixture("rich.img"),
            "cp", "/etc/fstab",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .success();
    assert!(tmp.path().join("fstab").exists());
}

#[test]
fn cp_file_to_new_path() {
    let tmp = TempDir::new().unwrap();
    let dest = tmp.path().join("my_fstab");
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source", &fixture("rich.img"),
            "cp", "/etc/fstab",
            dest.to_str().unwrap(),
        ])
        .assert()
        .success();
    assert!(dest.exists());
    let content = std::fs::read_to_string(&dest).unwrap();
    assert!(content.contains("ext4"));
}

#[test]
fn cp_recursive_directory() {
    let tmp = TempDir::new().unwrap();
    let dest = tmp.path().join("etc_copy");
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source", &fixture("rich.img"),
            "cp", "-r", "/etc",
            dest.to_str().unwrap(),
        ])
        .assert()
        .success();
    assert!(dest.join("fstab").exists());
    assert!(dest.join("passwd").exists());
}

#[test]
fn cp_directory_without_recursive_fails() {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source", &fixture("rich.img"),
            "cp", "/etc",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("-r"));
}

#[test]
fn cp_nonexistent_src_exits_3() {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source", &fixture("rich.img"),
            "cp", "/no/such/file",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .code(3);
}
