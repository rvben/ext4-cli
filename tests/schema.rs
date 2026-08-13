use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn schema_command_produces_valid_json() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let value: serde_json::Value =
        serde_json::from_slice(&output).expect("schema must be valid JSON");
    assert_eq!(value["clispec"].as_str(), Some("0.3"));
    assert_eq!(value["name"].as_str(), Some("ext4"));
    assert!(value["version"].is_string());
    assert!(value["commands"].is_array());
}

#[test]
fn schema_validates_against_clispec_v03() {
    let schema_bytes = include_bytes!("fixtures/clispec-v0.3.json");
    let schema_value: serde_json::Value = serde_json::from_slice(schema_bytes).unwrap();
    let validator = jsonschema::validator_for(&schema_value).expect("clispec schema must compile");

    let output = Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let doc: serde_json::Value = serde_json::from_slice(&output).unwrap();
    if let Err(e) = validator.validate(&doc) {
        panic!("schema output does not validate against clispec v0.3: {e}");
    }
}

#[test]
fn schema_works_without_source() {
    Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success();
}

#[test]
fn schema_mentioned_in_help() {
    Command::cargo_bin("ext4")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("schema"));
}

#[test]
fn schema_has_all_commands_with_effects() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let doc: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let commands = doc["commands"].as_array().unwrap();
    assert!(!commands.is_empty());
    for cmd in commands {
        assert!(
            cmd["effects"].is_string(),
            "Command {} is missing effects",
            cmd["name"]
        );
    }
}

#[test]
fn schema_classifies_output_and_mutation_capabilities() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let doc: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let commands = doc["commands"].as_array().unwrap();
    let command = |name: &str| commands.iter().find(|c| c["name"] == name).unwrap();

    assert_eq!(command("cat")["output_kind"], "opaque");
    assert_eq!(command("cp")["effects"], "idempotent");
    assert_eq!(command("ls")["cardinality"], "unbounded");
    assert_eq!(command("ls")["pagination"]["style"], "offset");
}

#[test]
fn schema_has_error_kinds_with_exit_codes() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let doc: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let errors = doc["errors"].as_array().unwrap();
    assert!(!errors.is_empty());
    for err in errors {
        assert!(err["kind"].is_string());
        assert!(err["exit_code"].is_number());
    }
}

#[test]
fn schema_has_global_args() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let doc: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(doc["global_args"].is_array());
    let global_args = doc["global_args"].as_array().unwrap();
    let names: Vec<&str> = global_args
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert!(
        names.contains(&"--output"),
        "global_args must include --output"
    );
}
