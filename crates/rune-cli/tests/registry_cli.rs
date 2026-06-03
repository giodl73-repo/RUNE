use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn check_registry_reports_workspace_registry() {
    let fixture = fixture_path("semantic_registry_workspace.json");
    let expected = include_str!("fixtures/semantic_registry_workspace.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-registry");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn inspect_registry_reports_workspace_registry_collections() {
    let fixture = fixture_path("semantic_registry_workspace.json");
    let expected = include_str!("fixtures/semantic_registry_workspace.inspect.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-registry");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_registry_rejects_duplicate_collection_ref() {
    let fixture = fixture_path("semantic_registry_duplicate_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-REGISTRY-003"));
}

#[test]
fn check_registry_rejects_runtime_capability() {
    let fixture = fixture_path("semantic_registry_runtime_blocked.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-REGISTRY-007"));
}

#[test]
fn check_registry_rejects_unknown_profile() {
    let fixture = fixture_path("semantic_registry_unknown_profile.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-REGISTRY-006"));
    assert!(stderr.contains("example.product_profile"));
}

#[test]
fn check_registry_rejects_collection_source_ref_mismatch() {
    let fixture = fixture_path("semantic_registry_mismatched_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-REGISTRY-005"));
    assert!(stderr.contains("collection source ref mismatch"));
}

#[test]
fn inspect_registry_rejects_collection_source_ref_mismatch() {
    let fixture = fixture_path("semantic_registry_mismatched_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-registry", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-REGISTRY-005"));
}

#[test]
fn check_registry_requires_fixture_flag() {
    let fixture = fixture_path("semantic_registry_workspace.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-registry", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune check-registry --fixture <path>"));
}

#[test]
fn inspect_registry_requires_fixture_flag() {
    let fixture = fixture_path("semantic_registry_workspace.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect-registry", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect-registry");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune inspect-registry --fixture <path>"));
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
