use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn check_reports_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_customer_descriptor.json");
    let expected = include_str!("fixtures/annotated_customer.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_reports_command_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_create_customer_command_descriptor.json");
    let expected = include_str!("fixtures/annotated_create_customer_command.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_reports_event_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_customer_created_event_descriptor.json");
    let expected = include_str!("fixtures/annotated_customer_created_event.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_reports_state_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_customer_lifecycle_state_descriptor.json");
    let expected = include_str!("fixtures/annotated_customer_lifecycle_state.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_reports_artifact_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_contract_evidence_artifact_descriptor.json");
    let expected = include_str!("fixtures/annotated_contract_evidence_artifact.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_reports_source_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_contract_source_reference_descriptor.json");
    let expected = include_str!("fixtures/annotated_contract_source_reference.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_reports_evidence_descriptor_profile_compatibility() {
    let fixture = fixture_path("annotated_contract_verification_evidence_descriptor.json");
    let expected = include_str!("fixtures/annotated_contract_verification_evidence.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_rejects_profile_unsupported_version() {
    let fixture = fixture_path("unsupported_version_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-CHECK-007"));
}

#[test]
fn check_rejects_profile_unsupported_kind() {
    let fixture = fixture_path("other_kind_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-CHECK-003"));
}

#[test]
fn check_collection_reports_profile_compatibility() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected = include_str!("fixtures/known_contract_descriptor_collection.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_collection_rejects_missing_collection_id() {
    let fixture = fixture_path("missing_collection_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-CHECK-001"));
}

#[test]
fn check_collection_rejects_duplicate_descriptor_ids() {
    let fixture = fixture_path("duplicate_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-CHECK-008"));
}

#[test]
fn check_collection_rejects_profile_unsupported_descriptor_kind() {
    let fixture = fixture_path("other_kind_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-CHECK-003"));
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
