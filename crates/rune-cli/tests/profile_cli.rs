use std::process::Command;

#[test]
fn profile_list_emits_approved_profile_catalog() {
    let expected = include_str!("fixtures/profile_list.json");

    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["profile", "list"])
        .output()
        .expect("run rune-cli profile list");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert_eq!(normalize_newlines(&stdout), normalize_newlines(expected));
}

#[test]
fn profile_rejects_unknown_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_rune-cli"))
        .args(["profile", "show"])
        .output()
        .expect("run rune-cli profile show");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("usage: rune profile list"));
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").trim_end().to_owned()
}
