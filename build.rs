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
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let icu_include_dirs = resolve_icu_include_dirs(&input_dir);

    println!("cargo:rerun-if-env-changed=CLANG");
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=XYO_ICU_ROOT");
    println!("cargo:rerun-if-env-changed=XYO_ICU_PREBUILT_DIR");
    println!("cargo:rerun-if-changed={}", input_dir.display());
    for include_dir in &icu_include_dirs {
        println!("cargo:rerun-if-changed={}", include_dir.display());
    }

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
            &icu_include_dirs,
            &source,
        );
    }

    write_embedded_bitcodes_rs(&output_bc_dir, &out_dir.join("embedded_bitcodes.rs"));
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

    sources.sort();
    sources
}

fn write_embedded_bitcodes_rs(output_bc_dir: &Path, destination: &Path) {
    let mut bitcode_paths = fs::read_dir(output_bc_dir)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", output_bc_dir.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension() == Some(OsStr::new("bc"))).then_some(path)
        })
        .collect::<Vec<_>>();

    bitcode_paths.sort();

    let mut contents = String::from("pub static EMBEDDED_BITCODES: &[(&str, &[u8])] = &[\n");
    for path in bitcode_paths {
        let name = path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_else(|| panic!("invalid bitcode file name: {}", path.display()));
        let include_path = path.display().to_string();
        contents.push_str(&format!(
            "    ({name:?}, include_bytes!({include_path:?})),\n"
        ));
    }
    contents.push_str("];\n");

    fs::write(destination, contents)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", destination.display()));
}

fn resolve_icu_include_dirs(input_dir: &Path) -> Vec<PathBuf> {
    let mut include_dirs = Vec::new();

    if let Ok(prebuilt_dir) = env::var("XYO_ICU_PREBUILT_DIR") {
        let include_dir = PathBuf::from(prebuilt_dir).join("include");
        if include_dir.join("unicode").join("ucasemap.h").is_file() {
            include_dirs.push(include_dir);
        }
    }

    if let Some(icu_root) = resolve_icu_root(input_dir) {
        let include_dir = icu_root.join("source").join("common");
        if include_dir.join("unicode").join("ucasemap.h").is_file()
            && !include_dirs.iter().any(|existing| existing == &include_dir)
        {
            include_dirs.push(include_dir);
        }
    }

    if include_dirs.is_empty() {
        panic!(
            "ICU headers not found; set XYO_ICU_PREBUILT_DIR to a prebuilt ICU install or XYO_ICU_ROOT to an ICU source tree"
        );
    }

    include_dirs
}

fn resolve_icu_root(input_dir: &Path) -> Option<PathBuf> {
    if let Ok(root) = env::var("XYO_ICU_ROOT") {
        let path = PathBuf::from(root);
        if path.is_dir() {
            return Some(path);
        }
    }

    let default_root = input_dir.join("lib").join("icu");
    if default_root.is_dir() {
        Some(default_root)
    } else {
        None
    }
}

fn resolve_clang() -> Option<PathBuf> {
    if let Ok(clang) = env::var("CLANG") {
        if command_exists(&clang) {
            return Some(PathBuf::from(clang));
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

    for clang in ["clang-21", "clang-20", "clang-19", "clang-18", "clang-17"] {
        if command_exists(clang) {
            return Some(PathBuf::from(clang));
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

    if let Ok(output) = Command::new("llvm-config").arg("--bindir").output() {
        if output.status.success() {
            let bindir = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            return (!bindir.is_empty()).then_some(PathBuf::from(bindir));
        }
    }

    for llvm_config in [
        "llvm-config-21",
        "llvm-config-20",
        "llvm-config-19",
        "llvm-config-18",
        "llvm-config-17",
    ] {
        let Ok(output) = Command::new(llvm_config).arg("--bindir").output() else {
            continue;
        };
        if !output.status.success() {
            continue;
        }

        let bindir = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        return (!bindir.is_empty()).then_some(PathBuf::from(bindir));
    }

    None
}

fn compile_source(
    clang: &Path,
    input_dir: &Path,
    output_bc_dir: &Path,
    output_ll_dir: &Path,
    icu_include_dirs: &[PathBuf],
    source: &Path,
) {
    let stem = source
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or_else(|| panic!("invalid source file name: {}", source.display()));

    let bc_output = output_bc_dir.join(format!("{stem}.bc"));
    let ll_output = output_ll_dir.join(format!("{stem}.ll"));
    let mut include_args = vec![
        OsStr::new("-I").to_os_string(),
        input_dir.join("lib").into_os_string(),
    ];
    for include_dir in icu_include_dirs {
        include_args.push(OsStr::new("-I").to_os_string());
        include_args.push(include_dir.as_os_str().to_os_string());
    }

    let mut bc_args = vec![
        OsStr::new("-emit-llvm").to_os_string(),
        OsStr::new("-c").to_os_string(),
        OsStr::new("-O3").to_os_string(),
    ];
    bc_args.extend(include_args.iter().cloned());
    bc_args.push(source.as_os_str().to_os_string());
    bc_args.push(OsStr::new("-o").to_os_string());
    bc_args.push(bc_output.into_os_string());

    run_clang(clang, bc_args);

    let mut ll_args = vec![
        OsStr::new("-S").to_os_string(),
        OsStr::new("-emit-llvm").to_os_string(),
        OsStr::new("-O3").to_os_string(),
    ];
    ll_args.extend(include_args);
    ll_args.push(source.as_os_str().to_os_string());
    ll_args.push(OsStr::new("-o").to_os_string());
    ll_args.push(ll_output.into_os_string());

    run_clang(clang, ll_args);
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
