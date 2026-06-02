use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn adapter_list_emits_approved_adapter_catalog() {
    let expected = include_str!("fixtures/adapter_list.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["adapter", "list"])
        .output()
        .expect("run rune-cli adapter list");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn adapt_collection_emits_review_packet() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected = include_str!("fixtures/known_contract_descriptor_collection.review_packet.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "adapt-collection",
            "--adapter",
            "rune.review_packet_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli adapt-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn adapt_collection_rejects_unknown_adapter() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "adapt-collection",
            "--adapter",
            "example.product_adapter",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli adapt-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-ADAPT-001"));
}

#[test]
fn adapt_collection_rejects_malformed_collection() {
    let fixture = fixture_path("missing_collection_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "adapt-collection",
            "--adapter",
            "rune.review_packet_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli adapt-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-ADAPT-002"));
}

#[test]
fn adapt_collection_rejects_unsupported_descriptor_kind() {
    let fixture = fixture_path("other_kind_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "adapt-collection",
            "--adapter",
            "rune.review_packet_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli adapt-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-ADAPT-003"));
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").trim_end().to_owned()
}
