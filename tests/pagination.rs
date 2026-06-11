use assert_cmd::Command;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn ls_limit_restricts_output() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "--output",
            "json",
            "ls",
            "--limit",
            "2",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let items = value["items"].as_array().unwrap();
    assert!(items.len() <= 2, "limit 2 should return at most 2 items");
    assert!(value["total"].as_u64().unwrap() >= items.len() as u64);
    assert_eq!(value["limit"].as_u64(), Some(2));
}

#[test]
fn ls_offset_skips_entries() {
    let all_output = Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "--output", "json", "ls"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let all: serde_json::Value = serde_json::from_slice(&all_output).unwrap();
    let all_items = all["items"].as_array().unwrap();

    if all_items.len() < 2 {
        return; // not enough items to test offset
    }

    let offset_output = Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "--output",
            "json",
            "ls",
            "--offset",
            "1",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let offset_val: serde_json::Value = serde_json::from_slice(&offset_output).unwrap();
    let offset_items = offset_val["items"].as_array().unwrap();

    assert_eq!(offset_val["offset"].as_u64(), Some(1));
    assert_eq!(offset_items.len(), all_items.len() - 1);
    assert_eq!(offset_items[0]["name"], all_items[1]["name"]);
}

#[test]
fn ls_fields_filters_output() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args([
            "--source",
            &fixture("rich.img"),
            "--output",
            "json",
            "ls",
            "--fields",
            "name,size",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let items = value["items"].as_array().unwrap();
    if !items.is_empty() {
        let item = &items[0];
        assert!(item["name"].is_string());
        assert!(item["size"].is_number());
        assert!(item.get("uid").is_none(), "uid should be filtered out");
        assert!(item.get("mode").is_none(), "mode should be filtered out");
    }
}

#[test]
fn ls_total_is_present_in_json_envelope() {
    let output = Command::cargo_bin("ext4")
        .unwrap()
        .args(["--source", &fixture("rich.img"), "--output", "json", "ls"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(value["total"].is_number());
    assert!(value["offset"].is_number());
}
