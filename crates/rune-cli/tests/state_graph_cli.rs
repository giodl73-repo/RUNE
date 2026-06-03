use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn check_state_graph_reports_workspace_graph() {
    let fixture = fixture_path("state_graph_workspace.json");
    let registry = fixture_path("semantic_registry_workspace.json");
    let expected = include_str!("fixtures/state_graph_workspace.check.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-state-graph",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-state-graph");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn check_state_graph_rejects_unknown_node_descriptor() {
    let output = run_check_state_graph("state_graph_unknown_descriptor.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-STATE-003"));
}

#[test]
fn check_state_graph_rejects_unknown_transition_node() {
    let output = run_check_state_graph("state_graph_unknown_node.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-STATE-004"));
}

#[test]
fn check_state_graph_rejects_unsupported_transition_descriptor() {
    let output = run_check_state_graph("state_graph_unsupported_transition.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-STATE-005"));
    assert!(stderr.contains("example.customer"));
}

#[test]
fn check_state_graph_rejects_live_state_requests() {
    let output = run_check_state_graph("state_graph_live_blocked.json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-STATE-006"));
}

#[test]
fn check_state_graph_requires_registry_flag() {
    let fixture = fixture_path("state_graph_workspace.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["check-state-graph", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli check-state-graph");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune check-state-graph --fixture <path> --registry <path>"));
}

fn run_check_state_graph(name: &str) -> std::process::Output {
    let fixture = fixture_path(name);
    let registry = fixture_path("semantic_registry_workspace.json");

    Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "check-state-graph",
            "--fixture",
            fixture.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
        ])
        .output()
        .expect("run rune-cli check-state-graph")
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
