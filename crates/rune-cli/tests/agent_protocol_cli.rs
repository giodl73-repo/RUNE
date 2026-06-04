use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn check_agent_protocol_reports_registry_describe_request() {
    let fixture = fixture_path("agent_protocol_registry_describe.json");
    let registry = fixture_path("semantic_registry_workspace.json");
    let expected = include_str!("fixtures/agent_protocol_registry_describe.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-agent-protocol",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-agent-protocol");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_agent_protocol_accepts_read_first_success_fixtures() {
    for name in [
        "agent_protocol_descriptor_get.json",
        "agent_protocol_compatibility_check.json",
    ] {
        let output = run_check_agent_protocol(name);

        assert!(output.status.success(), "{name} should validate");
    }
}

#[test]
fn check_agent_protocol_rejects_unknown_operation() {
    let output = run_check_agent_protocol("agent_protocol_unknown_operation.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-AGENT-001"));
}

#[test]
fn check_agent_protocol_rejects_missing_capability() {
    let output = run_check_agent_protocol("agent_protocol_missing_capability.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-AGENT-002"));
}

#[test]
fn check_agent_protocol_rejects_mutating_operation() {
    let output = run_check_agent_protocol("agent_protocol_mutating_operation.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-AGENT-003"));
}

#[test]
fn check_agent_protocol_rejects_unknown_ref() {
    let output = run_check_agent_protocol("agent_protocol_unknown_ref.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-AGENT-004"));
}

#[test]
fn check_agent_protocol_rejects_restricted_data() {
    let output = run_check_agent_protocol("agent_protocol_restricted_data.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-AGENT-005"));
}

#[test]
fn check_agent_protocol_requires_registry_flag() {
    let fixture = fixture_path("agent_protocol_registry_describe.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-agent-protocol",
            "--fixture",
            fixture.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-agent-protocol");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune check-agent-protocol --fixture <path> --registry <path>"));
}

fn run_check_agent_protocol(name: &str) -> std::process::Output {
    let fixture = fixture_path(name);
    let registry = fixture_path("semantic_registry_workspace.json");

    Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-agent-protocol",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-agent-protocol")
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
