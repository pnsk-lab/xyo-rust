use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let bitcodes_dir = manifest_dir.join("bitcodes");
    let input_dir = bitcodes_dir.join("c");
    let output_bc_dir = bitcodes_dir.join("bc");
    let output_ll_dir = bitcodes_dir.join("ll");

    let clang = resolve_clang().unwrap_or_else(|| {
        panic!(
            "clang is required to build bitcodes; set CLANG or LLVM_CONFIG_PATH, or put clang on PATH"
        )
    });

    fs::create_dir_all(&output_bc_dir).unwrap();
    fs::create_dir_all(&output_ll_dir).unwrap();

    for source in top_level_sources(&input_dir) {
        compile_source(
            &clang,
            &input_dir,
            &output_bc_dir,
            &output_ll_dir,
            &source,
        );
    }
}

fn top_level_sources(dir: &Path) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    let entries = fs::read_dir(dir).unwrap();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("c")) {
            sources.push(path);
        }
    }

    sources
}

fn resolve_clang() -> Option<PathBuf> {
    if let Ok(clang) = env::var("CLANG") {
        let path = PathBuf::from(clang);
        if path.is_file() {
            return Some(path);
        }
    }

    if command_exists("clang") {
        return Some(PathBuf::from("clang"));
    }

    if let Some(bin_dir) = llvm_bindir() {
        let clang = bin_dir.join("clang");
        if clang.is_file() {
            return Some(clang);
        }

        let clang_exe = bin_dir.join("clang.exe");
        if clang_exe.is_file() {
            return Some(clang_exe);
        }
    }

    None
}

fn command_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

fn llvm_bindir() -> Option<PathBuf> {
    if let Ok(path) = env::var("LLVM_CONFIG_PATH") {
        let path = PathBuf::from(path);
        return path.parent().map(Path::to_path_buf);
    }

    let output = Command::new("llvm-config")
        .arg("--bindir")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let bindir = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if bindir.is_empty() {
        None
    } else {
        Some(PathBuf::from(bindir))
    }
}

fn compile_source(
    clang: &Path,
    input_dir: &Path,
    output_bc_dir: &Path,
    output_ll_dir: &Path,
    source: &Path,
) {
    let stem = source
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or_else(|| panic!("invalid source file name: {}", source.display()));

    let bc_output = output_bc_dir.join(format!("{stem}.bc"));
    let ll_output = output_ll_dir.join(format!("{stem}.ll"));
    let include_dir = input_dir.join("lib");

    run_clang(
        clang,
        [
            "-emit-llvm",
            "-c",
            "-O3",
            "-I",
            include_dir.to_str().unwrap(),
            source.to_str().unwrap(),
            "-o",
            bc_output.to_str().unwrap(),
        ],
    );

    run_clang(
        clang,
        [
            "-S",
            "-emit-llvm",
            "-O3",
            "-I",
            include_dir.to_str().unwrap(),
            source.to_str().unwrap(),
            "-o",
            ll_output.to_str().unwrap(),
        ],
    );
}

fn run_clang<I, S>(clang: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(clang)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("failed to execute {}: {err}", clang.display()));

    if !output.status.success() {
        panic!(
            "clang failed: {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
