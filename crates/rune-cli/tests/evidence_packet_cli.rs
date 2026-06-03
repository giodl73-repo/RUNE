use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn check_evidence_packet_reports_diagnostic_packet() {
    let fixture = fixture_path("evidence_packet_diagnostic.json");
    let registry = fixture_path("semantic_registry_workspace.json");
    let expected = include_str!("fixtures/evidence_packet_diagnostic.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-evidence-packet",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-evidence-packet");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_evidence_packet_accepts_all_retained_packet_families() {
    for name in [
        "evidence_packet_validation.json",
        "evidence_packet_trace.json",
        "evidence_packet_health.json",
        "evidence_packet_audit.json",
    ] {
        let output = run_check_evidence_packet(name);

        assert!(output.status.success(), "{name} should validate");
    }
}

#[test]
fn check_evidence_packet_rejects_missing_identity() {
    let output = run_check_evidence_packet("evidence_packet_missing_identity.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-001"));
}

#[test]
fn check_evidence_packet_rejects_unsupported_kind() {
    let output = run_check_evidence_packet("evidence_packet_unsupported_kind.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-002"));
}

#[test]
fn check_evidence_packet_rejects_unknown_descriptor_ref() {
    let output = run_check_evidence_packet("evidence_packet_unknown_descriptor.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-003"));
}

#[test]
fn check_evidence_packet_rejects_unsupported_status() {
    let output = run_check_evidence_packet("evidence_packet_unsupported_status.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-004"));
}

#[test]
fn check_evidence_packet_rejects_audit_without_capability_decision() {
    let output = run_check_evidence_packet("evidence_packet_audit_missing_decision.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-005"));
}

#[test]
fn check_evidence_packet_rejects_mismatched_registry_ref() {
    let output = run_check_evidence_packet("evidence_packet_mismatched_registry.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-006"));
}

#[test]
fn check_evidence_packet_rejects_unknown_evidence_ref() {
    let output = run_check_evidence_packet("evidence_packet_unknown_evidence_ref.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-EVIDENCE-007"));
}

#[test]
fn check_evidence_packet_requires_registry_flag() {
    let fixture = fixture_path("evidence_packet_diagnostic.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-evidence-packet",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-evidence-packet");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("usage: rune check-evidence-packet --fixture <path> --registry <path>")
    );
}

fn run_check_evidence_packet(name: &str) -> std::process::Output {
    let fixture = fixture_path(name);
    let registry = fixture_path("semantic_registry_workspace.json");

    Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-evidence-packet",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-evidence-packet")
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
