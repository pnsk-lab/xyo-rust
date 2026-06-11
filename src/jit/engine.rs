use std::{
    env,
    ffi::CString,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};

use inkwell::{
    OptimizationLevel,
    execution_engine::ExecutionEngine,
    execution_engine::JitFunction,
    llvm_sys::support::LLVMLoadLibraryPermanently,
    module::Module,
    passes::PassBuilderOptions,
    support::load_visible_symbols,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
};

use crate::{
    compiler::types::{Builders, CostumeInfo, DynamicKind, DynamicStruct, SpriteStruct, StringStruct},
    jit::memory::SectionMemoryManager,
};
use llvm_sys::execution_engine::LLVMGetGlobalValueAddress;

type ThreadThunk = unsafe extern "C" fn(*mut SpriteStruct);

struct LibrarySpec {
    display_name: &'static str,
    prefer_bundled: bool,
    exact_names: &'static [&'static str],
    bundled_prefixes: &'static [&'static str],
}

#[cfg(target_os = "linux")]
const LIBRARIES: &[LibrarySpec] = &[
    LibrarySpec {
        display_name: "libc.so.6",
        prefer_bundled: false,
        exact_names: &["libc.so.6"],
        bundled_prefixes: &[],
    },
    LibrarySpec {
        display_name: "libm.so.6",
        prefer_bundled: false,
        exact_names: &["libm.so.6"],
        bundled_prefixes: &[],
    },
    LibrarySpec {
        display_name: "libicudata.so",
        prefer_bundled: true,
        exact_names: &["libicudata.so"],
        bundled_prefixes: &[],
    },
    LibrarySpec {
        display_name: "libicuuc.so",
        prefer_bundled: true,
        exact_names: &["libicuuc.so"],
        bundled_prefixes: &[],
    },
];

#[cfg(target_os = "windows")]
const LIBRARIES: &[LibrarySpec] = &[
    LibrarySpec {
        display_name: "kernel32.dll",
        prefer_bundled: false,
        exact_names: &["kernel32.dll"],
        bundled_prefixes: &[],
    },
    LibrarySpec {
        display_name: "ucrtbase.dll",
        prefer_bundled: false,
        exact_names: &["ucrtbase.dll"],
        bundled_prefixes: &[],
    },
    LibrarySpec {
        display_name: "icudt*.dll",
        prefer_bundled: true,
        exact_names: &[],
        bundled_prefixes: &["icudt"],
    },
    LibrarySpec {
        display_name: "icuuc*.dll",
        prefer_bundled: true,
        exact_names: &[],
        bundled_prefixes: &["icuuc"],
    },
];

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
const LIBRARIES: &[LibrarySpec] = &[];

#[repr(transparent)]
#[derive(Copy, Clone)]
struct SendMutPtr<T>(*mut T);

unsafe impl<T> Send for SendMutPtr<T> {}

impl<T> SendMutPtr<T> {
    fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }

    unsafe fn as_mut<'a>(self) -> &'a mut T {
        unsafe { &mut *self.0 }
    }
}

pub fn run<'ctx>(builders: &Builders<'ctx>, thread_functions: &[String]) {
    load_libraries();

    let target_machine = prepare_module(&builders.module);
    optimize_module(&builders.module, &target_machine);
    builders
        .module
        .verify()
        .unwrap_or_else(|err| panic!("generated module is invalid:\n{err}\n{}", builders.module.to_string()));

    load_visible_symbols();
    unsafe {
        llvm_sys::execution_engine::LLVMLinkInMCJIT();
    }
    let execution_engine = builders
        .module
        .create_mcjit_execution_engine_with_memory_manager(
            SectionMemoryManager::new(),
            OptimizationLevel::None,
            CodeModel::JITDefault,
            false,
            false,
        )
        .unwrap_or_else(|err| panic!("failed to create MCJIT execution engine: {err}"));

    let mut costume_storage = vec![
        CostumeInfo {
            width: 120.0,
            height: 90.0,
        },
        CostumeInfo {
            width: 80.0,
            height: 60.0,
        },
    ]
    .into_boxed_slice();
    let costume_ptr = costume_storage.as_mut_ptr();
    let costume_count = costume_storage.len() as i64;
    let costume_addr = costume_ptr as usize;

    for function_name in thread_functions {
        eprintln!("{function_name} has started");

        let function: JitFunction<'_, ThreadThunk> = unsafe { execution_engine.get_function(function_name) }
            .unwrap_or_else(|err| panic!("failed to find JIT function {function_name}: {err}"));
        let function = unsafe { function.as_raw() };

        let mut state = SpriteStruct::default();
        state.sprite_size = 100.0;
        state.sprite_costume_id = 1;
        state.sprite_costumes = costume_addr as *mut CostumeInfo;
        state.sprite_costume_number = costume_count;
        let state_ptr = SendMutPtr::new(&mut state as *mut SpriteStruct);

        let handle = std::thread::spawn(move || unsafe {
            let state = state_ptr.as_mut();
            function(state);
        });

        let started_at = Instant::now();

        while !handle.is_finished() {
            println!("{:?}", state);
            thread::sleep(Duration::from_millis(33));
        }
        handle.join().unwrap_or_else(|err| std::panic::resume_unwind(err));
        println!("\nelasped: {:?}", Instant::now() - started_at);
        println!("final state: {:?}", state);
        print_global_variables(builders, &execution_engine);
    }
}

fn print_global_variables(builders: &Builders<'_>, execution_engine: &ExecutionEngine<'_>) {
    if builders.global_variable_metadata.is_empty() {
        println!("global variables: <none>");
        return;
    }

    println!("global variables:");
    let mut variables = builders.global_variable_metadata.clone();
    variables.sort_by_key(|variable| variable.variable_idx);

    for variable in variables {
        let global_name = format!("global_{}", variable.variable_idx);
        let value = unsafe { read_global_variable(execution_engine, &global_name) };
        println!("  {} ({}): {}", variable.display_name, variable.variable_id, value);
    }
}

unsafe fn read_global_variable(execution_engine: &ExecutionEngine<'_>, global_name: &str) -> String {
    let name = CString::new(global_name).unwrap_or_else(|_| panic!("global name contains NUL byte: {global_name}"));
    let global_addr = unsafe { LLVMGetGlobalValueAddress(execution_engine.as_mut_ptr(), name.as_ptr()) };
    if global_addr == 0 {
        return "<unavailable>".to_string();
    }

    let dynamic_ptr = unsafe { *(global_addr as *const *const DynamicStruct) };
    if dynamic_ptr.is_null() {
        return "<null>".to_string();
    }

    let dynamic = unsafe { &*dynamic_ptr };
    unsafe { format_dynamic(dynamic) }
}

unsafe fn format_dynamic(dynamic: &DynamicStruct) -> String {
    if dynamic.pointer.is_null() {
        return "<null payload>".to_string();
    }

    match dynamic.kind {
        DynamicKind::String => unsafe { format_string(dynamic.pointer.cast::<StringStruct>()) },
        DynamicKind::Number => unsafe { (*(dynamic.pointer.cast::<f64>())).to_string() },
        DynamicKind::Bool => unsafe { (*(dynamic.pointer.cast::<bool>())).to_string() },
    }
}

unsafe fn format_string(value: *const StringStruct) -> String {
    if value.is_null() {
        return "<null string>".to_string();
    }

    let value = unsafe { &*value };
    if value.container.is_null() {
        return String::new();
    }

    let chars = unsafe { std::slice::from_raw_parts(value.container, value.length as usize) };
    String::from_utf16_lossy(chars)
}

fn prepare_module<'ctx>(module: &Module<'ctx>) -> TargetMachine {
    Target::initialize_native(&InitializationConfig::default()).expect("failed to initialize native LLVM target");

    let target_triple = TargetMachine::get_default_triple();
    let target =
        Target::from_triple(&target_triple).unwrap_or_else(|err| panic!("failed to resolve target from triple: {err}"));
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &TargetMachine::get_host_cpu_name().to_string(),
            &TargetMachine::get_host_cpu_features().to_string(),
            OptimizationLevel::Aggressive,
            RelocMode::PIC,
            CodeModel::JITDefault,
        )
        .expect("failed to create native target machine");

    module.set_triple(&target_triple);
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());
    target_machine
}

fn optimize_module<'ctx>(module: &Module<'ctx>, target_machine: &TargetMachine) {
    let pass_builder_options = PassBuilderOptions::create();
    pass_builder_options.set_loop_interleaving(true);
    pass_builder_options.set_loop_vectorization(true);
    pass_builder_options.set_loop_slp_vectorization(true);
    pass_builder_options.set_loop_unrolling(true);
    pass_builder_options.set_forget_all_scev_in_loop_unroll(true);

    module
        .run_passes("default<O3>", target_machine, pass_builder_options)
        .unwrap_or_else(|err| panic!("failed to optimize module for MCJIT: {err}"));
}

fn load_libraries() {
    let search_dirs = library_search_dirs();

    for spec in LIBRARIES {
        load_library(spec, &search_dirs);
    }
}

fn load_library(spec: &LibrarySpec, search_dirs: &[PathBuf]) {
    if spec.prefer_bundled {
        for path in bundled_candidates(spec, search_dirs) {
            if try_load_library(&path) {
                return;
            }
        }
    }

    for name in spec.exact_names {
        if try_load_library(Path::new(name)) {
            return;
        }
    }

    let tried_paths = if spec.prefer_bundled && !search_dirs.is_empty() {
        let tried = bundled_candidates(spec, search_dirs)
            .into_iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        let tried = if tried.is_empty() {
            String::from("<none>")
        } else {
            tried.join(", ")
        };
        format!("; tried bundled paths: {tried}")
    } else {
        String::new()
    };

    panic!("failed to load {}{}", spec.display_name, tried_paths);
}

fn try_load_library(path: &Path) -> bool {
    let path = CString::new(path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| panic!("library path contains NUL byte: {}", path.display()));

    unsafe { LLVMLoadLibraryPermanently(path.as_ptr()) == 0 }
}

fn library_search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(dir) = env::var("XYO_JIT_LIBRARY_DIR") {
        push_search_dir(&mut dirs, PathBuf::from(dir));
    }

    if let Ok(prefix) = env::var("XYO_ICU_RUNTIME_DIR") {
        let prefix = PathBuf::from(prefix);
        push_search_dir(&mut dirs, prefix.join("bin"));
        push_search_dir(&mut dirs, prefix.join("lib"));
        push_search_dir(&mut dirs, prefix);
    }

    if let Ok(current_exe) = env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            push_search_dir(&mut dirs, parent.join("lib"));
            push_search_dir(&mut dirs, parent.to_path_buf());

            if parent.file_name().and_then(|name| name.to_str()) == Some("bin") {
                if let Some(grand_parent) = parent.parent() {
                    push_search_dir(&mut dirs, grand_parent.join("lib"));
                }
            }
        }
    }

    dirs
}

fn push_search_dir(dirs: &mut Vec<PathBuf>, dir: PathBuf) {
    if dir.as_os_str().is_empty() || dirs.iter().any(|existing| existing == &dir) {
        return;
    }

    dirs.push(dir);
}

fn bundled_candidates(spec: &LibrarySpec, search_dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    for dir in search_dirs {
        for name in spec.exact_names {
            let path = dir.join(name);
            if path.is_file() && !candidates.iter().any(|existing| existing == &path) {
                candidates.push(path);
            }
        }

        for prefix in spec.bundled_prefixes {
            for path in find_library_matches(dir, prefix) {
                if !candidates.iter().any(|existing| existing == &path) {
                    candidates.push(path);
                }
            }
        }
    }

    candidates
}

fn find_library_matches(dir: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return matches,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if path.is_file()
            && file_name.starts_with(prefix)
            && file_name.ends_with(".dll")
            && !file_name.ends_with("d.dll")
        {
            matches.push(path);
        }
    }

    matches.sort();
    matches
}
