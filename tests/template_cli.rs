use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
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
fn json_command_with_example_file_works() {
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
fn template_failure_case() {
    let output = Command::new(env!("CARGO_BIN_EXE_xyo"))
        // TODO: replace with the scenario you want to validate.
        .arg("run")
        .arg(fixture("does-not-exist.sb3"))
        .output()
        .expect("failed to run `xyo run`");

    // TODO: customize expected exit code and stderr/stdout assertions.
    assert!(!output.status.success());
}
