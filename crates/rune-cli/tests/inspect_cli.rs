use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn inspect_fixture_emits_neutral_descriptor_json() {
    let fixture = fixture_path("valid_descriptor.json");
    let expected = include_str!("fixtures/valid_descriptor.inspection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn inspect_fixture_fails_closed_without_id() {
    let fixture = fixture_path("missing_id_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-INSP-001"));
}

#[test]
fn inspect_collection_fixture_emits_neutral_collection_json() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected = include_str!("fixtures/known_contract_descriptor_collection.inspection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-collection", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn inspect_collection_fixture_fails_closed_without_collection_id() {
    let fixture = fixture_path("missing_collection_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-collection", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-INSP-001"));
}

#[test]
fn inspect_collection_fixture_fails_closed_without_collection_version() {
    let fixture = fixture_path("missing_collection_version_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-collection", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-INSP-002"));
}

#[test]
fn inspect_collection_fixture_fails_closed_with_duplicate_descriptor_ids() {
    let fixture = fixture_path("duplicate_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-collection", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-INSP-003"));
}

#[test]
fn inventory_collection_fixture_emits_kind_counts() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected = include_str!("fixtures/known_contract_descriptor_collection.inventory.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "inventory-collection",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli inventory-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn inventory_collection_fixture_fails_closed_without_collection_id() {
    let fixture = fixture_path("missing_collection_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "inventory-collection",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli inventory-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-INV-001"));
}

#[test]
fn inventory_collection_fixture_fails_closed_with_duplicate_descriptor_ids() {
    let fixture = fixture_path("duplicate_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "inventory-collection",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli inventory-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-INV-003"));
}

#[test]
fn discover_manifest_emits_merged_collection_json() {
    let manifest = fixture_path("adopter_discovery_manifest.json");
    let expected = include_str!("fixtures/adopter_discovered_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["discover", "--manifest", manifest.to_str().unwrap()])
        .output()
        .expect("run rune-cli discover");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn discover_manifest_fails_closed_without_manifest_id() {
    let manifest = fixture_path("missing_manifest_id_discovery_manifest.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["discover", "--manifest", manifest.to_str().unwrap()])
        .output()
        .expect("run rune-cli discover");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-DISC-001"));
}

#[test]
fn discover_manifest_fails_closed_with_unsupported_source_kind() {
    let manifest = fixture_path("unsupported_source_kind_discovery_manifest.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["discover", "--manifest", manifest.to_str().unwrap()])
        .output()
        .expect("run rune-cli discover");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-DISC-005"));
}

#[test]
fn discover_manifest_fails_closed_with_duplicate_descriptor_ids() {
    let manifest = fixture_path("duplicate_descriptor_discovery_manifest.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["discover", "--manifest", manifest.to_str().unwrap()])
        .output()
        .expect("run rune-cli discover");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-DISC-007"));
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
