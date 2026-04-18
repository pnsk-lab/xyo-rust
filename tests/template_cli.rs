use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn help_command_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_xyo"))
        .arg("--help")
        .output()
        .expect("failed to run xyo --help");

    assert!(output.status.success());
}

#[test]
fn json_command_with_fixture_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_xyo"))
        .arg("json")
        .arg(fixture("simple.sb3"))
        .output()
        .expect("failed to run `xyo json`");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains('{'),
        "expected JSON-like output, got: {stdout}"
    );
}

#[test]
fn stats_command_with_fixture_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_xyo"))
        .arg("stats")
        .arg(fixture("simple.sb3"))
        .output()
        .expect("failed to run `xyo stats`");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Block Number: 0"),
        "unexpected output: {stdout}"
    );
}

#[test]
fn run_command_with_fixture_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_xyo"))
        .arg("run")
        .arg(fixture("simple.sb3"))
        .output()
        .expect("failed to run `xyo run`");

    assert!(output.status.success());
}

#[test]
fn missing_fixture_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_xyo"))
        .arg("run")
        .arg(fixture("does-not-exist.sb3"))
        .output()
        .expect("failed to run `xyo run`");

    assert!(!output.status.success());
}
