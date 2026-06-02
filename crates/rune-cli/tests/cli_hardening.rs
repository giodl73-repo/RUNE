use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn status_reports_current_v1_surfaces() {
    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .output()
        .expect("run rune-cli status");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("RUNE v1 surface"));
    assert!(stdout.contains("rune-adapters"));
    assert!(stdout.contains("rune.documentation_packet_json"));
    assert!(stdout.contains("rune.data_contract_json"));
    assert!(stdout.contains("rune.review_packet_json"));
}

#[test]
fn unknown_command_fails_with_command_name() {
    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["nonesuch"])
        .output()
        .expect("run rune-cli unknown command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("unknown command: nonesuch"));
}

#[test]
fn inspect_requires_fixture_flag() {
    let fixture = fixture_path("valid_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect without fixture flag");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune inspect --fixture <path>"));
}

#[test]
fn inspect_rejects_malformed_json_with_diagnostic() {
    let fixture = fixture_path("malformed_descriptor.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["inspect", "--fixture", fixture.to_str().unwrap()])
        .output()
        .expect("run rune-cli inspect malformed json");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("RUNE-INSP-900"));
    assert!(stderr.contains("error parsing fixture JSON"));
}

#[test]
fn adapt_collection_rejects_invalid_argument_order() {
    let fixture = fixture_path("known_contract_descriptor_collection.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args([
            "adapt-collection",
            "--fixture",
            fixture.to_str().unwrap(),
            "--adapter",
            "rune.review_packet_json",
        ])
        .output()
        .expect("run rune-cli adapt-collection with invalid order");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains(
        "usage: rune adapt-collection --adapter rune.review_packet_json --fixture <path>"
    ));
}

#[test]
fn adapter_rejects_unknown_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["adapter", "show"])
        .output()
        .expect("run rune-cli adapter show");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune adapter list"));
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}
