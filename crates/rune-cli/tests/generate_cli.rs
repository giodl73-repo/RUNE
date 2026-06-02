use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn generate_neutral_profile_emits_artifact_with_profile_metadata() {
    let fixture = fixture_path("valid_descriptor.json");
    let expected = include_str!("fixtures/valid_descriptor.neutral_descriptor_artifact.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_customer_descriptor() {
    let fixture = fixture_path("annotated_customer_descriptor.json");
    let expected = include_str!("fixtures/annotated_customer.neutral_descriptor_artifact.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_command_descriptor() {
    let fixture = fixture_path("annotated_create_customer_command_descriptor.json");
    let expected =
        include_str!("fixtures/annotated_create_customer_command.neutral_descriptor_artifact.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_event_descriptor() {
    let fixture = fixture_path("annotated_customer_created_event_descriptor.json");
    let expected =
        include_str!("fixtures/annotated_customer_created_event.neutral_descriptor_artifact.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_state_descriptor() {
    let fixture = fixture_path("annotated_customer_lifecycle_state_descriptor.json");
    let expected = include_str!(
        "fixtures/annotated_customer_lifecycle_state.neutral_descriptor_artifact.json"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_artifact_descriptor() {
    let fixture = fixture_path("annotated_contract_evidence_artifact_descriptor.json");
    let expected = include_str!(
        "fixtures/annotated_contract_evidence_artifact.neutral_descriptor_artifact.json"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_source_descriptor() {
    let fixture = fixture_path("annotated_contract_source_reference_descriptor.json");
    let expected = include_str!(
        "fixtures/annotated_contract_source_reference.neutral_descriptor_artifact.json"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_matches_annotated_evidence_descriptor() {
    let fixture = fixture_path("annotated_contract_verification_evidence_descriptor.json");
    let expected = include_str!(
        "fixtures/annotated_contract_verification_evidence.neutral_descriptor_artifact.json"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_documentation_packet_profile_emits_descriptor_packet() {
    let fixture = fixture_path("annotated_customer_descriptor.json");
    let expected = include_str!("fixtures/annotated_customer.documentation_packet.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.documentation_packet_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_neutral_profile_fails_closed_without_id() {
    let fixture = fixture_path("missing_id_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-GEN-001"));
}

#[test]
fn generate_rejects_unknown_profile() {
    let fixture = fixture_path("valid_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "example.product_profile",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-GEN-003"));
}

#[test]
fn generate_neutral_profile_rejects_unsupported_kind() {
    let fixture = fixture_path("unsupported_kind_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-GEN-003"));
    assert!(stderr.contains("unsupported descriptor kind"));
}

#[test]
fn generate_neutral_profile_rejects_profile_unsupported_other_kind() {
    let fixture = fixture_path("other_kind_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-GEN-003"));
    assert!(stderr.contains("does not support descriptor kind"));
}

#[test]
fn generate_neutral_profile_rejects_profile_unsupported_version() {
    let fixture = fixture_path("unsupported_version_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-GEN-007"));
    assert!(stderr.contains("does not support descriptor version"));
}

#[test]
fn generate_collection_neutral_profile_emits_collection_artifact() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected = include_str!(
        "fixtures/known_contract_descriptor_collection.neutral_descriptor_artifact.json"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_collection_documentation_packet_profile_emits_collection_packet() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");
    let expected =
        include_str!("fixtures/known_contract_descriptor_collection.documentation_packet.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate-collection",
            "--profile",
            "rune.documentation_packet_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate-collection");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn generate_collection_rejects_missing_collection_id() {
    let fixture = fixture_path("missing_collection_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-GEN-001"));
}

#[test]
fn generate_collection_rejects_duplicate_descriptor_ids() {
    let fixture = fixture_path("duplicate_id_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-GEN-008"));
}

#[test]
fn generate_collection_rejects_profile_unsupported_descriptor_kind() {
    let fixture = fixture_path("other_kind_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate-collection",
            "--profile",
            "rune.neutral_descriptor_json",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-GEN-003"));
}

#[test]
fn generate_collection_rejects_unknown_profile() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "generate-collection",
            "--profile",
            "example.product_profile",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli generate-collection");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COLL-GEN-003"));
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
