use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn evidence_collection_fixture_emits_bundle() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected =
        include_str!("fixtures/known_contract_descriptor_collection.evidence_bundle.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "evidence-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli evidence-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn evidence_collection_manifest_emits_discovered_bundle() {
    let manifest = fixture_path("adopter_discovery_manifest.json");
    let expected = include_str!("fixtures/adopter_discovered_collection.evidence_bundle.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "evidence-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--manifest",
            manifest.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli evidence-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn evidence_collection_rejects_unknown_profile() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "evidence-collection",
            "--profile",
            "example.product_profile",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli evidence-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVID-003"));
}

#[test]
fn evidence_collection_rejects_malformed_collection() {
    let fixture = fixture_path("missing_collection_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "evidence-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli evidence-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVID-001"));
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
