use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn check_compatibility_reports_collection_profile_compatibility() {
    let fixture = fixture_path("compatibility_collection_profile.json");
    let registry = fixture_path("semantic_registry_workspace.json");
    let expected = include_str!("fixtures/compatibility_collection_profile.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-compatibility",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-compatibility");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_compatibility_accepts_retained_report_fixtures() {
    for name in [
        "compatibility_collection_adapter.json",
        "compatibility_registry_state_graph.json",
    ] {
        let output = run_check_compatibility(name);

        assert!(output.status.success(), "{name} should validate");
    }
}

#[test]
fn check_compatibility_rejects_unknown_source() {
    let output = run_check_compatibility("compatibility_unknown_source.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COMPAT-001"));
}

#[test]
fn check_compatibility_rejects_unknown_target() {
    let output = run_check_compatibility("compatibility_unknown_target.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COMPAT-002"));
}

#[test]
fn check_compatibility_rejects_unsupported_version() {
    let output = run_check_compatibility("compatibility_unsupported_version.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COMPAT-003"));
}

#[test]
fn check_compatibility_rejects_unsupported_concept_claim() {
    let output = run_check_compatibility("compatibility_unsupported_concept.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COMPAT-004"));
}

#[test]
fn check_compatibility_rejects_unapproved_degradation() {
    let output = run_check_compatibility("compatibility_degraded_unapproved.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COMPAT-005"));
}

#[test]
fn check_compatibility_rejects_runtime_host_negotiation() {
    let output = run_check_compatibility("compatibility_runtime_host_blocked.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-COMPAT-006"));
}

#[test]
fn check_compatibility_requires_registry_flag() {
    let fixture = fixture_path("compatibility_collection_profile.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-compatibility",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-compatibility");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune check-compatibility --fixture <path> --registry <path>"));
}

fn run_check_compatibility(name: &str) -> std::process::Output {
    let fixture = fixture_path(name);
    let registry = fixture_path("semantic_registry_workspace.json");

    Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-compatibility",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-compatibility")
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
