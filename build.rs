use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct BuildTools {
    clang: PathBuf,
    llvm_link: Option<PathBuf>,
    llvm_dis: Option<PathBuf>,
}

struct IcuVendor {
    root: PathBuf,
    sources: Vec<PathBuf>,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let bitcodes_dir = manifest_dir.join("bitcodes");
    let input_dir = bitcodes_dir.join("c");
    let output_bc_dir = bitcodes_dir.join("bc");
    let output_ll_dir = bitcodes_dir.join("ll");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let scratch_dir = out_dir.join("bitcode-build");
    let icu_dir = input_dir.join("lib").join("icu");

    println!("cargo:rerun-if-env-changed=CLANG");
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=XYO_EMBED_ICU_BITCODE");
    println!("cargo:rerun-if-env-changed=XYO_ICU_ROOT");
    println!("cargo:rerun-if-env-changed=XYO_ICU_PREBUILT_DIR");
    println!("cargo:rerun-if-changed={}", input_dir.display());
    println!("cargo:rerun-if-changed={}", icu_dir.display());

    let tools = resolve_build_tools();
    let icu_vendor = discover_icu_vendor(&input_dir);

    fs::create_dir_all(&output_bc_dir).unwrap();
    fs::create_dir_all(&output_ll_dir).unwrap();
    fs::create_dir_all(&scratch_dir).unwrap();

    for source in top_level_sources(&input_dir) {
        compile_source(
            &tools,
            &input_dir,
            &output_bc_dir,
            &output_ll_dir,
            &scratch_dir,
            icu_vendor.as_ref(),
            &source,
        );
    }

    write_embedded_bitcodes_rs(&output_bc_dir, &out_dir.join("embedded_bitcodes.rs"));
}

fn discover_icu_vendor(input_dir: &Path) -> Option<IcuVendor> {
    let root = resolve_icu_root(input_dir)?;
    if !icu_source_headers_available(&root) {
        return None;
    }

    let manifest = root.join("xyo-icu-sources.txt");
    if !manifest.is_file() {
        return None;
    }
    println!("cargo:rerun-if-changed={}", manifest.display());

    let contents = fs::read_to_string(&manifest).unwrap_or_else(|err| {
        panic!(
            "vendored ICU was detected at {}, but {} could not be read: {err}",
            root.display(),
            manifest.display()
        )
    });

    let sources: Vec<PathBuf> = contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(PathBuf::from)
        .collect();

    if sources.is_empty() {
        panic!(
            "vendored ICU was detected at {}, but {} does not list any sources",
            root.display(),
            manifest.display()
        );
    }

    Some(IcuVendor { root, sources })
}

fn resolve_icu_root(input_dir: &Path) -> Option<PathBuf> {
    if let Some(root) = env::var_os("XYO_ICU_ROOT") {
        return Some(PathBuf::from(root));
    }

    let default_root = input_dir.join("lib").join("icu");
    if default_root.exists() {
        Some(default_root)
    } else {
        None
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
        let escaped = path.display().to_string().replace('\\', "\\\\");
        contents.push_str(&format!(
            "    ({name:?}, include_bytes!(r\"{escaped}\")),\n"
        ));
    }
    contents.push_str("];\n");

    fs::write(destination, contents)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", destination.display()));
}

fn resolve_build_tools() -> BuildTools {
    let clang = resolve_clang().unwrap_or_else(|| {
        panic!(
            "clang is required to build bitcodes; set CLANG or LLVM_CONFIG_PATH, or put clang on PATH"
        )
    });

    BuildTools {
        clang,
        llvm_link: resolve_llvm_tool("llvm-link"),
        llvm_dis: resolve_llvm_tool("llvm-dis"),
    }
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

fn resolve_llvm_tool(tool: &str) -> Option<PathBuf> {
    if command_exists(tool) {
        return Some(PathBuf::from(tool));
    }

    if let Some(versioned_tool) = versioned_llvm_tool_name(tool) {
        if command_exists(&versioned_tool) {
            return Some(PathBuf::from(versioned_tool));
        }
    }

    if let Some(bin_dir) = llvm_bindir() {
        let binary = bin_dir.join(tool);
        if binary.is_file() {
            return Some(binary);
        }

        let binary_exe = bin_dir.join(format!("{tool}.exe"));
        if binary_exe.is_file() {
            return Some(binary_exe);
        }

        if let Some(versioned_tool) = versioned_llvm_tool_name(tool) {
            let versioned_binary = bin_dir.join(&versioned_tool);
            if versioned_binary.is_file() {
                return Some(versioned_binary);
            }

            let versioned_binary_exe = bin_dir.join(format!("{versioned_tool}.exe"));
            if versioned_binary_exe.is_file() {
                return Some(versioned_binary_exe);
            }
        }
    }

    None
}

fn versioned_llvm_tool_name(tool: &str) -> Option<String> {
    let llvm_config = env::var("LLVM_CONFIG_PATH").ok()?;
    let file_name = Path::new(&llvm_config).file_name()?.to_str()?;
    let suffix = file_name.strip_prefix("llvm-config")?;
    if suffix.is_empty() {
        None
    } else {
        Some(format!("{tool}{suffix}"))
    }
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

    let output = Command::new("llvm-config").arg("--bindir").output().ok()?;

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
    tools: &BuildTools,
    input_dir: &Path,
    output_bc_dir: &Path,
    output_ll_dir: &Path,
    scratch_dir: &Path,
    icu_vendor: Option<&IcuVendor>,
    source: &Path,
) {
    let stem = source
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or_else(|| panic!("invalid source file name: {}", source.display()));

    let bc_output = output_bc_dir.join(format!("{stem}.bc"));
    let ll_output = output_ll_dir.join(format!("{stem}.ll"));

    if stem == "str" {
        let include_dirs = to_lower_include_dirs(input_dir, icu_vendor);
        compile_to_lower(
            tools,
            input_dir,
            scratch_dir,
            icu_vendor,
            &include_dirs,
            source,
            &bc_output,
            &ll_output,
        );
        return;
    }

    compile_plain_source(
        tools,
        input_dir,
        source,
        &bc_output,
        &ll_output,
        false,
        &[],
        &[],
    );
}

fn compile_to_lower(
    tools: &BuildTools,
    input_dir: &Path,
    scratch_dir: &Path,
    icu_vendor: Option<&IcuVendor>,
    include_dirs: &[PathBuf],
    source: &Path,
    bc_output: &Path,
    ll_output: &Path,
) {
    if let Some(icu_vendor) = icu_vendor.filter(|_| should_embed_icu_bitcode()) {
        let raw_bc = scratch_dir.join("to_lower.raw.bc");
        compile_plain_source(
            tools,
            input_dir,
            source,
            &raw_bc,
            ll_output,
            true,
            &include_dirs,
            &[],
        );

        let icu_bc_dir = scratch_dir.join("icu");
        fs::create_dir_all(&icu_bc_dir).unwrap();

        let mut link_inputs = vec![raw_bc];
        for icu_source in &icu_vendor.sources {
            let compiled = compile_icu_source(tools, icu_vendor, &icu_bc_dir, icu_source);
            link_inputs.push(compiled);
        }

        llvm_link(tools, &link_inputs, bc_output);
        llvm_dis(tools, bc_output, ll_output);
    } else {
        compile_plain_source(
            tools,
            input_dir,
            source,
            bc_output,
            ll_output,
            false,
            &include_dirs,
            &[],
        );
    }
}

fn should_embed_icu_bitcode() -> bool {
    matches!(
        env::var("XYO_EMBED_ICU_BITCODE")
            .ok()
            .as_deref()
            .map(str::trim),
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON")
    )
}

fn to_lower_include_dirs(input_dir: &Path, icu_vendor: Option<&IcuVendor>) -> Vec<PathBuf> {
    if let Some(vendor) = icu_vendor {
        return icu_include_dirs(&vendor.root);
    }

    if let Some(root) =
        resolve_icu_root(input_dir).filter(|root| icu_source_headers_available(root))
    {
        return icu_include_dirs(&root);
    }

    let prebuilt_root = env::var_os("XYO_ICU_PREBUILT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| input_dir.join("lib").join("icu-prebuilt"));
    let include_root = prebuilt_root.join("include");
    if icu_prebuilt_header_path(&prebuilt_root).is_file() {
        return vec![include_root];
    }

    if let Some(root) = resolve_icu_root(input_dir) {
        println!(
            "cargo:warning=ignoring incomplete ICU source tree at {}; expected {}",
            root.display(),
            icu_vendor_header_path(&root).display()
        );
    }

    panic!(
        "ICU headers for bitcodes/c/str.c were not found. \
set XYO_ICU_ROOT to a full ICU source tree, or build/install prebuilt ICU headers under {} \
(for example with ./tools/build_icu_prebuilt.sh or ./setup.sh).",
        include_root.display()
    );
}

fn compile_plain_source(
    tools: &BuildTools,
    input_dir: &Path,
    source: &Path,
    bc_output: &Path,
    ll_output: &Path,
    embed_icu: bool,
    extra_include_dirs: &[PathBuf],
    extra_clang_args: &[&str],
) {
    let include_dir = input_dir.join("lib");

    run_command(
        &tools.clang,
        clang_args(
            &include_dir,
            source,
            bc_output,
            true,
            embed_icu,
            extra_include_dirs,
            extra_clang_args,
        ),
    );

    run_command(
        &tools.clang,
        clang_args(
            &include_dir,
            source,
            ll_output,
            false,
            embed_icu,
            extra_include_dirs,
            extra_clang_args,
        ),
    );
}

fn compile_icu_source(
    tools: &BuildTools,
    vendor: &IcuVendor,
    output_dir: &Path,
    relative_source: &Path,
) -> PathBuf {
    let source = vendor.root.join(relative_source);
    if !source.is_file() {
        panic!(
            "ICU source listed in {} was not found: {}",
            vendor.root.join("xyo-icu-sources.txt").display(),
            source.display()
        );
    }

    let file_name = relative_source
        .to_string_lossy()
        .replace(['/', '\\'], "__")
        .replace('.', "_");
    let output = output_dir.join(format!("{file_name}.bc"));

    let mut args: Vec<OsString> = vec![
        OsString::from("-emit-llvm"),
        OsString::from("-c"),
        OsString::from("-O3"),
        OsString::from("-DU_STATIC_IMPLEMENTATION"),
    ];

    if is_cpp_source(&source) {
        args.push(OsString::from("-std=c++17"));
    }

    match icu_macro_for_source(relative_source) {
        Some(def) => args.push(OsString::from(format!("-D{def}"))),
        None => {}
    }

    for include_dir in icu_include_dirs(&vendor.root) {
        args.push(OsString::from("-I"));
        args.push(include_dir.into_os_string());
    }

    args.push(source.into_os_string());
    args.push(OsString::from("-o"));
    args.push(output.clone().into_os_string());

    run_command(&tools.clang, args);
    output
}

fn icu_include_dirs(root: &Path) -> Vec<PathBuf> {
    vec![
        root.join("source"),
        root.join("source").join("common"),
        root.join("source").join("i18n"),
    ]
}

fn icu_vendor_header_path(root: &Path) -> PathBuf {
    root.join("source")
        .join("common")
        .join("unicode")
        .join("ucasemap.h")
}

fn icu_source_headers_available(root: &Path) -> bool {
    root.join("source").is_dir() && icu_vendor_header_path(root).is_file()
}

fn icu_prebuilt_header_path(root: &Path) -> PathBuf {
    root.join("include").join("unicode").join("ucasemap.h")
}

fn icu_macro_for_source(relative_source: &Path) -> Option<&'static str> {
    let normalized = relative_source.to_string_lossy().replace('\\', "/");
    if normalized.starts_with("source/common/") {
        Some("U_COMMON_IMPLEMENTATION")
    } else if normalized.starts_with("source/i18n/") {
        Some("U_I18N_IMPLEMENTATION")
    } else {
        None
    }
}

fn is_cpp_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(OsStr::to_str),
        Some("cc" | "cp" | "cxx" | "cpp")
    )
}

fn llvm_link(tools: &BuildTools, inputs: &[PathBuf], output: &Path) {
    let llvm_link = tools.llvm_link.as_ref().unwrap_or_else(|| {
        panic!(
            "llvm-link is required to link self-contained bitcode; set LLVM_CONFIG_PATH or put llvm-link on PATH"
        )
    });
    let mut args: Vec<OsString> = vec![OsString::from("-o"), output.as_os_str().to_os_string()];
    for input in inputs {
        args.push(input.as_os_str().to_os_string());
    }
    run_command(llvm_link, args);
}

fn llvm_dis(tools: &BuildTools, input: &Path, output: &Path) {
    let llvm_dis = tools.llvm_dis.as_ref().unwrap_or_else(|| {
        panic!(
            "llvm-dis is required to emit readable LLVM IR; set LLVM_CONFIG_PATH or put llvm-dis on PATH"
        )
    });
    run_command(
        llvm_dis,
        vec![
            OsString::from("-o"),
            output.as_os_str().to_os_string(),
            input.as_os_str().to_os_string(),
        ],
    );
}

fn clang_args(
    include_dir: &Path,
    source: &Path,
    output: &Path,
    emit_bc: bool,
    embed_icu: bool,
    extra_include_dirs: &[PathBuf],
    extra_clang_args: &[&str],
) -> Vec<OsString> {
    let mut args = vec![OsString::from("-emit-llvm")];

    if emit_bc {
        args.push(OsString::from("-c"));
    } else {
        args.push(OsString::from("-S"));
    }

    args.push(OsString::from("-O3"));
    args.push(OsString::from("-I"));
    args.push(include_dir.as_os_str().to_os_string());

    for extra_include_dir in extra_include_dirs {
        args.push(OsString::from("-I"));
        args.push(extra_include_dir.as_os_str().to_os_string());
    }

    if embed_icu {
        args.push(OsString::from("-DXYO_EMBED_ICU"));
    }
    for extra_clang_arg in extra_clang_args {
        args.push(OsString::from(extra_clang_arg));
    }
    args.push(source.as_os_str().to_os_string());
    args.push(OsString::from("-o"));
    args.push(output.as_os_str().to_os_string());
    args
}

fn run_command<I, S>(program: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("failed to execute {}: {err}", program.display()));

    if !output.status.success() {
        panic!(
            "{} failed: {}\n{}",
            program.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
