use crate::engine::ir::{
    CloneTarget, ControlStopMode, Expr, MathOp, PenColorParam, Procedure, Program, SayExpr, Script,
    ScriptTrigger, SensingCurrentMenu, Stmt,
};
use crate::engine::runtime::{
    RuntimeState, STRING_TAG_BITS, STRING_TAG_MASK, ScriptEntry, encode_string_id, rt_change_var,
    rt_control_create_clone_of, rt_control_delete_this_clone, rt_control_stop, rt_control_wait,
    rt_count_executed_block, rt_data_add_to_list, rt_data_delete_all_of_list,
    rt_data_delete_of_list, rt_data_item_num_of_list, rt_data_item_of_list, rt_data_length_of_list,
    rt_data_list_contains_item, rt_data_replace_item_of_list, rt_event_broadcast_and_wait_value,
    rt_event_broadcast_value, rt_forever_should_continue, rt_forever_should_continue_warp,
    rt_get_var, rt_get_variables_ptr, rt_looks_costume_name, rt_looks_costume_number,
    rt_looks_hide, rt_looks_say_number, rt_looks_say_text, rt_looks_set_effect_to,
    rt_looks_set_size, rt_looks_show, rt_looks_switch_backdrop_to, rt_looks_switch_costume_to,
    rt_loop_should_continue, rt_loop_should_continue_warp, rt_motion_change_x, rt_motion_change_y,
    rt_motion_goto_xy, rt_motion_move_steps, rt_motion_set_direction, rt_motion_set_x,
    rt_motion_set_y, rt_motion_x_position, rt_motion_y_position, rt_music_set_tempo,
    rt_operator_add, rt_operator_contains, rt_operator_divide, rt_operator_equals,
    rt_operator_greater_than, rt_operator_join, rt_operator_length, rt_operator_less_than,
    rt_operator_letter_of, rt_operator_mathop, rt_operator_mod, rt_operator_multiply,
    rt_operator_round, rt_operator_subtract, rt_pen_clear, rt_pen_down, rt_pen_set_color,
    rt_pen_set_color_param, rt_pen_set_size, rt_pen_stamp, rt_pen_up, rt_random, rt_repeat_count,
    rt_sensing_answer, rt_sensing_ask_and_wait, rt_sensing_current, rt_sensing_days_since_2000,
    rt_sensing_key_pressed, rt_sensing_mouse_down, rt_sensing_mouse_x, rt_sensing_mouse_y,
    rt_sensing_of, rt_sensing_reset_timer, rt_sensing_timer, rt_sensing_touching_color,
    rt_sensing_touching_object, rt_set_var, rt_warp_enter, rt_warp_leave,
};
use anyhow::{Context, Result, anyhow, bail};
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context as LlvmContext;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::types::{BasicMetadataTypeEnum, FloatType, IntType, PointerType};
use inkwell::values::{BasicMetadataValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate, OptimizationLevel};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::Builder as TempDirBuilder;

mod layout;
mod runtime_bindings;

/// Number of loop iterations between full guard checks.
/// On non-guard iterations the loop runs without any extern "C" function
/// calls, eliminating register spill/reload overhead.  The full guard
/// (which checks stop_requested, step budget, time-slice yield, etc.) is
/// invoked once every LOOP_GUARD_INTERVAL iterations.
///
/// TurboWarp uses 100; we use 256 for power-of-two friendliness.
const LOOP_GUARD_INTERVAL: u64 = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JitOptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

impl JitOptimizationLevel {
    pub fn parse(raw: &str) -> Result<Self> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "O0" | "0" => Ok(Self::O0),
            "O1" | "1" => Ok(Self::O1),
            "O2" | "2" => Ok(Self::O2),
            "O3" | "3" | "AGGRESSIVE" => Ok(Self::O3),
            _ => bail!("invalid LLVM optimization level: {raw} (expected O0, O1, O2, or O3)"),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::O0 => "O0",
            Self::O1 => "O1",
            Self::O2 => "O2",
            Self::O3 => "O3",
        }
    }

    fn to_inkwell(self) -> OptimizationLevel {
        match self {
            Self::O0 => OptimizationLevel::None,
            Self::O1 => OptimizationLevel::Less,
            Self::O2 => OptimizationLevel::Default,
            Self::O3 => OptimizationLevel::Aggressive,
        }
    }
}

impl Default for JitOptimizationLevel {
    fn default() -> Self {
        Self::O3
    }
}

/// Represents a compiled Scratch program ready for execution
pub struct CompiledProgram {
    _native_module: Library,
    layout: CompilationLayout,
    script_functions: Vec<ScriptEntry>,
    program_info: ProgramInfo,
}

#[derive(Clone)]
struct ProgramInfo {
    target_names: Vec<String>,
}

impl CompiledProgram {
    /// Execute the compiled program with the given runtime state.
    ///
    /// Uses fiber-based cooperative concurrency so that all active scripts
    /// advance one yield-step per tick, matching Scratch's threading model.
    pub fn execute(&self, runtime_state: &mut RuntimeState) {
        runtime_state.install_scheduler(
            self.script_functions.clone(),
            self.layout.script_names_by_id.clone(),
            self.layout.broadcast_messages.clone(),
            self.layout.broadcast_targets.clone(),
            self.layout.key_press_options.clone(),
            self.layout.key_press_targets.clone(),
            self.layout.clone_targets.clone(),
            self.layout.script_target_ids.clone(),
            self.program_info.target_names.clone(),
            self.layout.target_count,
        );
        runtime_state.enqueue_scripts(&self.layout.entry_script_ids);

        // Run the concurrent tick-based scheduler.
        runtime_state.execute_concurrent();
    }
}

/// Compile a Scratch program into native code and load it
pub fn compile_and_load_program(program: &Program) -> Result<CompiledProgram> {
    compile_and_load_program_with_optimization(program, JitOptimizationLevel::default())
}

pub fn compile_and_load_program_with_optimization(
    program: &Program,
    optimization_level: JitOptimizationLevel,
) -> Result<CompiledProgram> {
    if program.scripts.is_empty() {
        bail!("no scripts to compile");
    }

    Target::initialize_native(&InitializationConfig::default())
        .map_err(|message| anyhow!("failed to initialize native LLVM target: {}", message))?;

    let context = LlvmContext::create();
    let module = context.create_module("scratch_native_runtime");
    let execution_engine = module
        .create_jit_execution_engine(optimization_level.to_inkwell())
        .map_err(|message| anyhow!("failed to create LLVM execution engine: {}", message))?;

    let runtime_functions = RuntimeFunctions::declare(&context, &module, &execution_engine);
    let mut compiler = JitCompiler::new(&context, &module, runtime_functions);
    let layout = compiler.compile_program(program)?;

    module
        .verify()
        .map_err(|message| anyhow!("generated invalid LLVM module: {}", message.to_string()))?;

    let (target_machine, triple) = create_host_target_machine(RelocMode::PIC, optimization_level)?;
    module.set_triple(&triple);
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());

    let artifact_dir = TempDirBuilder::new()
        .prefix("scratch-native-runtime-")
        .tempdir()
        .context("failed to create temporary directory for native build artifacts")?;
    let object_path = artifact_dir.path().join("program.o");
    let library_path = artifact_dir.path().join(native_library_filename());

    target_machine
        .write_to_file(&module, FileType::Object, &object_path)
        .map_err(|message| {
            anyhow!(
                "failed to write temporary object file {}: {}",
                object_path.display(),
                message
            )
        })?;
    link_native_shared_object(&object_path, &library_path)?;

    let native_module = unsafe {
        Library::new(&library_path).with_context(|| {
            format!(
                "failed to load native module generated at {}",
                library_path.display()
            )
        })?
    };
    let script_functions = load_script_functions(&native_module, &layout.script_names_by_id)?;

    Ok(CompiledProgram {
        _native_module: native_module,
        layout,
        script_functions,
        program_info: ProgramInfo {
            target_names: program.target_names.clone(),
        },
    })
}

pub fn execute_program(program: &Program, runtime_state: &mut RuntimeState) -> Result<()> {
    execute_program_with_optimization(program, runtime_state, JitOptimizationLevel::default())
}

pub fn execute_program_with_optimization(
    program: &Program,
    runtime_state: &mut RuntimeState,
    optimization_level: JitOptimizationLevel,
) -> Result<()> {
    let compiled = compile_and_load_program_with_optimization(program, optimization_level)?;
    runtime_state.install_scheduler(
        compiled.script_functions.clone(),
        compiled.layout.script_names_by_id.clone(),
        compiled.layout.broadcast_messages.clone(),
        compiled.layout.broadcast_targets.clone(),
        compiled.layout.key_press_options.clone(),
        compiled.layout.key_press_targets.clone(),
        compiled.layout.clone_targets.clone(),
        compiled.layout.script_target_ids.clone(),
        compiled.program_info.target_names.clone(),
        compiled.layout.target_count,
    );
    runtime_state.enqueue_scripts(&compiled.layout.entry_script_ids);
    runtime_state.execute_concurrent();
    Ok(())
}

pub fn execute_program_with_mode(
    program: &Program,
    runtime_state: &mut RuntimeState,
    native_async: bool,
) -> Result<()> {
    execute_program_with_mode_and_optimization(
        program,
        runtime_state,
        native_async,
        JitOptimizationLevel::default(),
    )
}

pub fn execute_program_with_mode_and_optimization(
    program: &Program,
    runtime_state: &mut RuntimeState,
    native_async: bool,
    optimization_level: JitOptimizationLevel,
) -> Result<()> {
    if native_async {
        return execute_program_with_optimization(program, runtime_state, optimization_level);
    }

    let compiled = compile_and_load_program_with_optimization(program, optimization_level)?;
    runtime_state.install_scheduler(
        compiled.script_functions.clone(),
        compiled.layout.script_names_by_id.clone(),
        compiled.layout.broadcast_messages.clone(),
        compiled.layout.broadcast_targets.clone(),
        compiled.layout.key_press_options.clone(),
        compiled.layout.key_press_targets.clone(),
        compiled.layout.clone_targets.clone(),
        compiled.layout.script_target_ids.clone(),
        compiled.program_info.target_names.clone(),
        compiled.layout.target_count,
    );
    runtime_state.enqueue_scripts(&compiled.layout.entry_script_ids);
    runtime_state.execute_serial();
    Ok(())
}

pub fn emit_native_object(program: &Program, output_path: &Path) -> Result<()> {
    emit_native_object_with_optimization(program, output_path, JitOptimizationLevel::default())
}

pub fn emit_native_object_with_optimization(
    program: &Program,
    output_path: &Path,
    optimization_level: JitOptimizationLevel,
) -> Result<()> {
    Target::initialize_native(&InitializationConfig::default())
        .map_err(|message| anyhow!("failed to initialize native LLVM target: {}", message))?;

    let context = LlvmContext::create();
    let module = context.create_module("scratch_native_runtime");
    // Reuse the existing declaration path that wires host runtime symbols.
    let execution_engine = module
        .create_jit_execution_engine(optimization_level.to_inkwell())
        .map_err(|message| anyhow!("failed to create LLVM execution engine: {}", message))?;
    let runtime_functions = RuntimeFunctions::declare(&context, &module, &execution_engine);
    let mut compiler = JitCompiler::new(&context, &module, runtime_functions);
    compiler.compile_program(program)?;

    module
        .verify()
        .map_err(|message| anyhow!("generated invalid LLVM module: {}", message.to_string()))?;

    let (target_machine, triple) =
        create_host_target_machine(RelocMode::Default, optimization_level)?;

    module.set_triple(&triple);
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create output directory for native object: {}",
                    parent.display()
                )
            })?;
        }
    }

    target_machine
        .write_to_file(&module, FileType::Object, output_path)
        .map_err(|message| {
            anyhow!(
                "failed to write native object file {}: {}",
                output_path.display(),
                message
            )
        })?;

    Ok(())
}

fn create_host_target_machine(
    reloc_mode: RelocMode,
    optimization_level: JitOptimizationLevel,
) -> Result<(TargetMachine, TargetTriple)> {
    let triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();
    let target = Target::from_triple(&triple)
        .map_err(|message| anyhow!("failed to resolve LLVM target triple: {}", message))?;
    let target_machine = target
        .create_target_machine(
            &triple,
            cpu.to_str().unwrap_or("generic"),
            features.to_str().unwrap_or(""),
            optimization_level.to_inkwell(),
            reloc_mode,
            CodeModel::Default,
        )
        .ok_or_else(|| anyhow!("failed to create LLVM target machine"))?;
    Ok((target_machine, triple))
}

fn native_library_filename() -> &'static str {
    #[cfg(target_os = "linux")]
    {
        "program.so"
    }
    #[cfg(target_os = "macos")]
    {
        "program.dylib"
    }
    #[cfg(target_os = "windows")]
    {
        "program.dll"
    }
}

fn link_native_shared_object(object_path: &Path, library_path: &Path) -> Result<()> {
    #[cfg(target_os = "linux")]
    let mut command = {
        let mut command = Command::new("cc");
        command
            .arg("-shared")
            .arg("-o")
            .arg(library_path)
            .arg(object_path);
        command
    };
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("cc");
        command
            .arg("-dynamiclib")
            .arg("-o")
            .arg(library_path)
            .arg(object_path);
        command
    };
    #[cfg(target_os = "windows")]
    {
        let _ = object_path;
        let _ = library_path;
        bail!("native backend currently does not support Windows");
    }

    let status = command.status().with_context(|| {
        format!(
            "failed to invoke system linker for native module {}",
            library_path.display()
        )
    })?;
    if !status.success() {
        bail!(
            "system linker failed while creating native module {} (status: {})",
            library_path.display(),
            status
        );
    }
    Ok(())
}

fn load_script_functions(native_module: &Library, symbols: &[String]) -> Result<Vec<ScriptEntry>> {
    let mut script_functions = Vec::with_capacity(symbols.len());
    for symbol_name in symbols {
        let mut c_symbol = symbol_name.as_bytes().to_vec();
        c_symbol.push(0);
        let function: Symbol<'_, ScriptEntry> = unsafe {
            native_module
                .get(&c_symbol)
                .with_context(|| format!("failed to resolve native symbol `{}`", symbol_name))?
        };
        script_functions.push(*function);
    }
    Ok(script_functions)
}

#[allow(dead_code)]
struct RuntimeFunctions<'ctx> {
    count_executed_block: FunctionValue<'ctx>,
    move_steps: FunctionValue<'ctx>,
    set_direction: FunctionValue<'ctx>,
    change_x: FunctionValue<'ctx>,
    change_y: FunctionValue<'ctx>,
    set_x: FunctionValue<'ctx>,
    set_y: FunctionValue<'ctx>,
    goto_xy: FunctionValue<'ctx>,
    get_var: FunctionValue<'ctx>,
    set_var: FunctionValue<'ctx>,
    change_var: FunctionValue<'ctx>,
    get_variables_ptr: FunctionValue<'ctx>,
    data_add_to_list: FunctionValue<'ctx>,
    data_delete_of_list: FunctionValue<'ctx>,
    data_delete_all_of_list: FunctionValue<'ctx>,
    data_replace_item_of_list: FunctionValue<'ctx>,
    data_list_contains_item: FunctionValue<'ctx>,
    say_number: FunctionValue<'ctx>,
    say_text: FunctionValue<'ctx>,
    looks_switch_costume_to: FunctionValue<'ctx>,
    looks_switch_backdrop_to: FunctionValue<'ctx>,
    looks_set_effect_to: FunctionValue<'ctx>,
    looks_set_size: FunctionValue<'ctx>,
    looks_costume_number: FunctionValue<'ctx>,
    looks_costume_name: FunctionValue<'ctx>,
    looks_hide: FunctionValue<'ctx>,
    looks_show: FunctionValue<'ctx>,
    music_set_tempo: FunctionValue<'ctx>,
    sensing_ask_and_wait: FunctionValue<'ctx>,
    sensing_answer: FunctionValue<'ctx>,
    sensing_of: FunctionValue<'ctx>,
    sensing_current: FunctionValue<'ctx>,
    sensing_timer: FunctionValue<'ctx>,
    sensing_days_since_2000: FunctionValue<'ctx>,
    sensing_touching_object: FunctionValue<'ctx>,
    sensing_touching_color: FunctionValue<'ctx>,
    sensing_reset_timer: FunctionValue<'ctx>,
    pen_down: FunctionValue<'ctx>,
    pen_up: FunctionValue<'ctx>,
    pen_clear: FunctionValue<'ctx>,
    pen_set_size: FunctionValue<'ctx>,
    pen_set_color: FunctionValue<'ctx>,
    pen_stamp: FunctionValue<'ctx>,
    pen_set_color_param: FunctionValue<'ctx>,
    control_create_clone: FunctionValue<'ctx>,
    control_delete_clone: FunctionValue<'ctx>,
    control_stop: FunctionValue<'ctx>,
    control_wait: FunctionValue<'ctx>,
    repeat_count: FunctionValue<'ctx>,
    operator_length: FunctionValue<'ctx>,
    operator_join: FunctionValue<'ctx>,
    operator_contains: FunctionValue<'ctx>,
    operator_round: FunctionValue<'ctx>,
    operator_letter_of: FunctionValue<'ctx>,
    operator_mathop: FunctionValue<'ctx>,
    operator_add: FunctionValue<'ctx>,
    operator_subtract: FunctionValue<'ctx>,
    operator_multiply: FunctionValue<'ctx>,
    operator_divide: FunctionValue<'ctx>,
    operator_mod: FunctionValue<'ctx>,
    operator_equals: FunctionValue<'ctx>,
    operator_greater_than: FunctionValue<'ctx>,
    operator_less_than: FunctionValue<'ctx>,
    data_item_of_list: FunctionValue<'ctx>,
    data_item_num_of_list: FunctionValue<'ctx>,
    data_length_of_list: FunctionValue<'ctx>,
    motion_x_position: FunctionValue<'ctx>,
    motion_y_position: FunctionValue<'ctx>,
    sensing_mouse_x: FunctionValue<'ctx>,
    sensing_mouse_y: FunctionValue<'ctx>,
    sensing_mouse_down: FunctionValue<'ctx>,
    sensing_key_pressed: FunctionValue<'ctx>,
    event_broadcast_value: FunctionValue<'ctx>,
    event_broadcast_wait_value: FunctionValue<'ctx>,
    forever_should_continue: FunctionValue<'ctx>,
    forever_should_continue_warp: FunctionValue<'ctx>,
    loop_should_continue: FunctionValue<'ctx>,
    loop_should_continue_warp: FunctionValue<'ctx>,
    warp_enter: FunctionValue<'ctx>,
    warp_leave: FunctionValue<'ctx>,
    random: FunctionValue<'ctx>,
}

struct CompilationLayout {
    script_names_by_id: Vec<String>,
    entry_script_ids: Vec<u64>,
    broadcast_messages: Vec<String>,
    broadcast_targets: Vec<Vec<u64>>,
    key_press_options: Vec<String>,
    key_press_targets: Vec<Vec<u64>>,
    clone_targets: Vec<Vec<u64>>,
    script_target_ids: Vec<u64>,
    target_count: usize,
}

struct JitCompiler<'ctx, 'm> {
    context: &'ctx LlvmContext,
    module: &'m Module<'ctx>,
    builder: Builder<'ctx>,
    runtime: RuntimeFunctions<'ctx>,
    f64_type: FloatType<'ctx>,
    i64_type: IntType<'ctx>,
    ptr_type: PointerType<'ctx>,
    script_functions: HashMap<String, FunctionValue<'ctx>>,
    script_id_by_name: HashMap<String, u64>,
    script_names_by_id: Vec<String>,
    target_index_by_name: HashMap<String, u64>,
    script_target_ids: Vec<u64>,
    procedure_functions: HashMap<usize, FunctionValue<'ctx>>,
    procedure_warp_flags: HashMap<usize, bool>,
    message_index_by_name: HashMap<String, u64>,
    broadcast_messages: Vec<String>,
    broadcast_targets: Vec<Vec<u64>>,
    key_press_options: Vec<String>,
    key_press_targets: Vec<Vec<u64>>,
    clone_targets: Vec<Vec<u64>>,
    current_fn: Option<FunctionValue<'ctx>>,
    current_proc_params: Vec<FloatValue<'ctx>>,
    current_is_procedure: bool,
    current_warp_mode: bool,
    /// Cached base pointer of the variables Vec data buffer.
    /// Set once at the entry of each function/procedure via
    /// `rt_get_variables_ptr`.  Used for inline GEP+load/store
    /// instead of calling `rt_get_var`/`rt_set_var` per access.
    cached_vars_ptr: Option<PointerValue<'ctx>>,
    string_counter: usize,
}

impl<'ctx, 'm> JitCompiler<'ctx, 'm> {
    fn new(
        context: &'ctx LlvmContext,
        module: &'m Module<'ctx>,
        runtime: RuntimeFunctions<'ctx>,
    ) -> Self {
        Self {
            context,
            module,
            builder: context.create_builder(),
            runtime,
            f64_type: context.f64_type(),
            i64_type: context.i64_type(),
            ptr_type: context.ptr_type(AddressSpace::default()),
            script_functions: HashMap::new(),
            script_id_by_name: HashMap::new(),
            script_names_by_id: Vec::new(),
            target_index_by_name: HashMap::new(),
            script_target_ids: Vec::new(),
            procedure_functions: HashMap::new(),
            procedure_warp_flags: HashMap::new(),
            message_index_by_name: HashMap::new(),
            broadcast_messages: Vec::new(),
            broadcast_targets: Vec::new(),
            key_press_options: Vec::new(),
            key_press_targets: Vec::new(),
            clone_targets: Vec::new(),
            current_fn: None,
            current_proc_params: Vec::new(),
            current_is_procedure: false,
            current_warp_mode: false,
            cached_vars_ptr: None,
            string_counter: 0,
        }
    }

    fn compile_script(&mut self, script: &Script) -> Result<()> {
        let function = self
            .script_functions
            .get(&script.name)
            .copied()
            .ok_or_else(|| anyhow!("missing function declaration for script {}", script.name))?;
        self.current_fn = Some(function);
        self.current_proc_params.clear();
        self.current_is_procedure = false;
        self.current_warp_mode = false;

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        let runtime_ptr = function
            .get_nth_param(0)
            .ok_or_else(|| anyhow!("missing runtime state argument in {}", script.name))?
            .into_pointer_value();

        // Cache the variables array base pointer for inline access.
        let vars_ptr = self
            .call_ptr(
                self.runtime.get_variables_ptr,
                &[runtime_ptr.into()],
                "vars_ptr",
            )?;
        self.cached_vars_ptr = Some(vars_ptr);

        for statement in &script.body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }

        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_return(None))?;
        }

        self.current_fn = None;
        self.current_is_procedure = false;
        self.current_warp_mode = false;
        Ok(())
    }

    fn compile_procedure(&mut self, index: usize, procedure: &Procedure) -> Result<()> {
        let function = self
            .procedure_functions
            .get(&index)
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "missing function declaration for procedure {}",
                    procedure.name
                )
            })?;

        self.current_fn = Some(function);
        self.current_is_procedure = true;
        self.current_warp_mode = procedure.warp;
        self.current_proc_params = (0..procedure.arg_names.len())
            .map(|arg_index| {
                function
                    .get_nth_param((arg_index + 1) as u32)
                    .unwrap_or_else(|| self.f64_type.const_zero().into())
                    .into_float_value()
            })
            .collect();

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        let runtime_ptr = function
            .get_nth_param(0)
            .ok_or_else(|| anyhow!("missing runtime state argument in {}", procedure.name))?
            .into_pointer_value();

        // Cache the variables array base pointer for inline access.
        let vars_ptr = self
            .call_ptr(
                self.runtime.get_variables_ptr,
                &[runtime_ptr.into()],
                "vars_ptr",
            )?;
        self.cached_vars_ptr = Some(vars_ptr);

        for statement in &procedure.body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }

        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_return(Some(&self.f64_type.const_zero())))?;
        }

        self.current_fn = None;
        self.current_proc_params.clear();
        self.current_is_procedure = false;
        self.current_warp_mode = false;
        Ok(())
    }

    fn current_loop_guard(&self) -> FunctionValue<'ctx> {
        // Warp mode is now tracked dynamically via RuntimeState::warp_depth.
        // The guard function checks warp_depth at runtime.
        self.runtime.loop_should_continue
    }

    fn current_forever_guard(&self) -> FunctionValue<'ctx> {
        // Warp mode is now tracked dynamically via RuntimeState::warp_depth.
        // The guard function checks warp_depth at runtime.
        self.runtime.forever_should_continue
    }

    /// Allocate a loop-local guard counter and return its stack pointer.
    /// The counter is initialised to `LOOP_GUARD_INTERVAL` so the first
    /// iteration always enters the fast path (no extern call).
    fn alloc_loop_guard_counter(&self, label: &str) -> Result<PointerValue<'ctx>> {
        let name = format!("{}.guard_ctr", label);
        let counter_ptr = self.build(self.builder.build_alloca(self.i64_type, &name))?;
        self.build(self.builder.build_store(
            counter_ptr,
            self.i64_type.const_int(LOOP_GUARD_INTERVAL, false),
        ))?;
        Ok(counter_ptr)
    }

    /// Emit an inline guard check that only calls the full extern guard
    /// function every `LOOP_GUARD_INTERVAL` iterations.  On all other
    /// iterations a simple counter decrement is performed — no extern
    /// function call, no register-spill overhead.
    ///
    /// Returns an i1 value indicating whether the loop should continue.
    fn emit_inline_guard(
        &self,
        runtime_ptr: PointerValue<'ctx>,
        guard_counter_ptr: PointerValue<'ctx>,
        guard_fn: FunctionValue<'ctx>,
        label: &str,
    ) -> Result<IntValue<'ctx>> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("inline guard emitted without active function"))?;

        let fast_bb = self
            .context
            .append_basic_block(function, &format!("{}.guard_fast", label));
        let slow_bb = self
            .context
            .append_basic_block(function, &format!("{}.guard_slow", label));
        let merge_bb = self
            .context
            .append_basic_block(function, &format!("{}.guard_merge", label));

        // Load and decrement the counter.
        let counter = self
            .build(self.builder.build_load(
                self.i64_type,
                guard_counter_ptr,
                &format!("{}.ctr", label),
            ))?
            .into_int_value();
        let decremented = self.build(self.builder.build_int_sub(
            counter,
            self.i64_type.const_int(1, false),
            &format!("{}.ctr_dec", label),
        ))?;
        self.build(self.builder.build_store(guard_counter_ptr, decremented))?;

        // If counter > 0 after decrement, take the fast path (continue loop
        // without any extern call).
        let need_full = self.build(self.builder.build_int_compare(
            IntPredicate::SLE,
            decremented,
            self.i64_type.const_zero(),
            &format!("{}.need_full", label),
        ))?;
        self.build(
            self.builder
                .build_conditional_branch(need_full, slow_bb, fast_bb),
        )?;

        // Fast path: no extern call, loop continues.
        self.builder.position_at_end(fast_bb);
        let true_val = self.context.bool_type().const_int(1, false);
        self.build(self.builder.build_unconditional_branch(merge_bb))?;

        // Slow path: reset counter and call full guard.
        self.builder.position_at_end(slow_bb);
        self.build(self.builder.build_store(
            guard_counter_ptr,
            self.i64_type.const_int(LOOP_GUARD_INTERVAL, false),
        ))?;
        // Also bump the block counter by LOOP_GUARD_INTERVAL to maintain
        // approximate statistics without per-statement overhead.
        self.call_void(self.runtime.count_executed_block, &[runtime_ptr.into()])?;
        let guard_result = self.call_i1(
            guard_fn,
            &[runtime_ptr.into()],
            &format!("{}.guard_call", label),
        )?;
        self.build(self.builder.build_unconditional_branch(merge_bb))?;
        let slow_bb_end = self.builder.get_insert_block().unwrap();

        // Merge: phi node selects the result.
        self.builder.position_at_end(merge_bb);
        let phi = self.build(
            self.builder
                .build_phi(self.context.bool_type(), &format!("{}.guard_phi", label)),
        )?;
        phi.add_incoming(&[(&true_val, fast_bb), (&guard_result, slow_bb_end)]);

        Ok(phi.as_basic_value().into_int_value())
    }

    fn compile_statement(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        statement: &Stmt,
    ) -> Result<()> {
        // NOTE: rt_count_executed_block removed from per-statement emission
        // for performance.  Block counting is now done inside the loop guard
        // functions (amortized over many iterations) to avoid the overhead of
        // an extern "C" function call on every single statement.
        match statement {
            Stmt::MotionMoveSteps(steps) => {
                let steps = self.compile_expr(runtime_ptr, steps)?;
                self.call_void(self.runtime.move_steps, &[runtime_ptr.into(), steps.into()])?;
            }
            Stmt::MotionSetDirection(direction) => {
                let direction = self.compile_expr(runtime_ptr, direction)?;
                self.call_void(
                    self.runtime.set_direction,
                    &[runtime_ptr.into(), direction.into()],
                )?;
            }
            Stmt::MotionChangeX(delta) => {
                let delta = self.compile_expr(runtime_ptr, delta)?;
                self.call_void(self.runtime.change_x, &[runtime_ptr.into(), delta.into()])?;
            }
            Stmt::MotionChangeY(delta) => {
                let delta = self.compile_expr(runtime_ptr, delta)?;
                self.call_void(self.runtime.change_y, &[runtime_ptr.into(), delta.into()])?;
            }
            Stmt::MotionSetX(x) => {
                let x = self.compile_expr(runtime_ptr, x)?;
                self.call_void(self.runtime.set_x, &[runtime_ptr.into(), x.into()])?;
            }
            Stmt::MotionSetY(y) => {
                let y = self.compile_expr(runtime_ptr, y)?;
                self.call_void(self.runtime.set_y, &[runtime_ptr.into(), y.into()])?;
            }
            Stmt::MotionGoToXY { x, y } => {
                let x = self.compile_expr(runtime_ptr, x)?;
                let y = self.compile_expr(runtime_ptr, y)?;
                self.call_void(
                    self.runtime.goto_xy,
                    &[runtime_ptr.into(), x.into(), y.into()],
                )?;
            }
            Stmt::DataSetVariable {
                variable_index,
                value,
            } => {
                let value = self.compile_expr(runtime_ptr, value)?;
                if let Some(vars_ptr) = self.cached_vars_ptr {
                    // Inline variable store: GEP into cached variables array
                    let gep = unsafe {
                        self.build(self.builder.build_in_bounds_gep(
                            self.f64_type,
                            vars_ptr,
                            &[self.i64_type.const_int(*variable_index as u64, false)],
                            "var.set.gep",
                        ))?
                    };
                    self.build(self.builder.build_store(gep, value))?;
                } else {
                    let index = self.i64_type.const_int(*variable_index as u64, false);
                    self.call_void(
                        self.runtime.set_var,
                        &[runtime_ptr.into(), index.into(), value.into()],
                    )?;
                }
            }
            Stmt::DataChangeVariable {
                variable_index,
                delta,
            } => {
                let index = self.i64_type.const_int(*variable_index as u64, false);
                let delta = self.compile_expr(runtime_ptr, delta)?;
                self.call_void(
                    self.runtime.change_var,
                    &[runtime_ptr.into(), index.into(), delta.into()],
                )?;
            }
            Stmt::DataReplaceListItem {
                list_index,
                index,
                item,
            } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                let index = self.compile_expr(runtime_ptr, index)?;
                let item = self.compile_expr(runtime_ptr, item)?;
                self.call_void(
                    self.runtime.data_replace_item_of_list,
                    &[
                        runtime_ptr.into(),
                        list_index.into(),
                        index.into(),
                        item.into(),
                    ],
                )?;
            }
            Stmt::DataAddToList { list_index, item } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                let item = self.compile_expr(runtime_ptr, item)?;
                self.call_void(
                    self.runtime.data_add_to_list,
                    &[runtime_ptr.into(), list_index.into(), item.into()],
                )?;
            }
            Stmt::DataDeleteListItem { list_index, index } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                let index = self.compile_expr(runtime_ptr, index)?;
                self.call_void(
                    self.runtime.data_delete_of_list,
                    &[runtime_ptr.into(), list_index.into(), index.into()],
                )?;
            }
            Stmt::DataDeleteAllOfList { list_index } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                self.call_void(
                    self.runtime.data_delete_all_of_list,
                    &[runtime_ptr.into(), list_index.into()],
                )?;
            }
            Stmt::LooksSwitchCostumeTo(costume) => {
                let costume = self.compile_expr(runtime_ptr, costume)?;
                self.call_void(
                    self.runtime.looks_switch_costume_to,
                    &[runtime_ptr.into(), costume.into()],
                )?;
            }
            Stmt::LooksSwitchBackdropTo(backdrop) => {
                let backdrop = self.compile_expr(runtime_ptr, backdrop)?;
                self.call_void(
                    self.runtime.looks_switch_backdrop_to,
                    &[runtime_ptr.into(), backdrop.into()],
                )?;
            }
            Stmt::LooksSetEffectTo { effect, value } => {
                let effect = self.compile_expr(runtime_ptr, effect)?;
                let value = self.compile_expr(runtime_ptr, value)?;
                self.call_void(
                    self.runtime.looks_set_effect_to,
                    &[runtime_ptr.into(), effect.into(), value.into()],
                )?;
            }
            Stmt::LooksSetSize(size) => {
                let size = self.compile_expr(runtime_ptr, size)?;
                self.call_void(
                    self.runtime.looks_set_size,
                    &[runtime_ptr.into(), size.into()],
                )?;
            }
            Stmt::LooksShow => {
                self.call_void(self.runtime.looks_show, &[runtime_ptr.into()])?;
            }
            Stmt::LooksSay(SayExpr::Text(text)) => {
                let global =
                    self.build(self.builder.build_global_string_ptr(
                        text,
                        &format!("say_text_{}", self.string_counter),
                    ))?;
                self.string_counter += 1;
                let pointer = global.as_pointer_value();
                self.call_void(self.runtime.say_text, &[runtime_ptr.into(), pointer.into()])?;
            }
            Stmt::LooksSay(SayExpr::Numeric(expr)) => {
                let value = self.compile_expr(runtime_ptr, expr)?;
                self.call_void(self.runtime.say_number, &[runtime_ptr.into(), value.into()])?;
            }
            Stmt::SoundPlay => {}
            Stmt::ControlRepeat { times, body } => self.compile_repeat(runtime_ptr, times, body)?,
            Stmt::ControlWait { duration } => {
                let duration = self.compile_expr(runtime_ptr, duration)?;
                self.call_void(
                    self.runtime.control_wait,
                    &[runtime_ptr.into(), duration.into()],
                )?;
            }
            Stmt::ControlWaitUntil { condition } => {
                self.compile_wait_until(runtime_ptr, condition)?;
            }
            Stmt::ControlForEach {
                variable_index,
                count,
                body,
            } => self.compile_for_each(runtime_ptr, *variable_index, count, body)?,
            Stmt::ControlForever { body } => self.compile_forever(runtime_ptr, body)?,
            Stmt::ControlRepeatUntil { condition, body } => {
                self.compile_repeat_until(runtime_ptr, condition, body)?
            }
            Stmt::ControlWhile { condition, body } => {
                self.compile_while(runtime_ptr, condition, body)?
            }
            Stmt::ControlIf {
                condition,
                then_body,
                else_body,
            } => self.compile_if(runtime_ptr, condition, then_body, else_body)?,
            Stmt::MotionSetRotationStyle => {}
            Stmt::DataShowVariable => {}
            Stmt::ControlStop { mode } => {
                let mode_value = self.i64_type.const_int(Self::stop_mode_code(*mode), false);
                self.call_void(
                    self.runtime.control_stop,
                    &[runtime_ptr.into(), mode_value.into()],
                )?;
                if !matches!(*mode, ControlStopMode::OtherScriptsInTarget) {
                    self.build_return_for_current_function()?;
                }
            }
            Stmt::ControlCreateCloneOf { target } => {
                let target_value = match target {
                    CloneTarget::Myself => self.i64_type.const_all_ones(),
                    CloneTarget::Target(index) => self.i64_type.const_int(*index as u64, false),
                };
                self.call_void(
                    self.runtime.control_create_clone,
                    &[runtime_ptr.into(), target_value.into()],
                )?;
            }
            Stmt::ControlDeleteThisClone => {
                self.call_void(self.runtime.control_delete_clone, &[runtime_ptr.into()])?;
                self.build_return_for_current_function()?;
            }
            Stmt::SensingAskAndWait { question } => {
                let question = self.compile_expr(runtime_ptr, question)?;
                self.call_void(
                    self.runtime.sensing_ask_and_wait,
                    &[runtime_ptr.into(), question.into()],
                )?;
            }
            Stmt::MusicSetTempo { tempo } => {
                let tempo = self.compile_expr(runtime_ptr, tempo)?;
                self.call_void(
                    self.runtime.music_set_tempo,
                    &[runtime_ptr.into(), tempo.into()],
                )?;
            }
            Stmt::SensingResetTimer => {
                self.call_void(self.runtime.sensing_reset_timer, &[runtime_ptr.into()])?;
            }
            Stmt::LooksHide => {
                self.call_void(self.runtime.looks_hide, &[runtime_ptr.into()])?;
            }
            Stmt::SensingSetDragMode => {}
            Stmt::TextToSpeechSpeakAndWait(words) => {
                // text2speech extension is currently treated as compatibility no-op.
                let _ = self.compile_expr(runtime_ptr, words)?;
            }
            Stmt::PenDown => {
                self.call_void(self.runtime.pen_down, &[runtime_ptr.into()])?;
            }
            Stmt::PenUp => {
                self.call_void(self.runtime.pen_up, &[runtime_ptr.into()])?;
            }
            Stmt::PenClear => {
                self.call_void(self.runtime.pen_clear, &[runtime_ptr.into()])?;
            }
            Stmt::PenSetSize(size) => {
                let size = self.compile_expr(runtime_ptr, size)?;
                self.call_void(
                    self.runtime.pen_set_size,
                    &[runtime_ptr.into(), size.into()],
                )?;
            }
            Stmt::PenSetColor(color) => {
                let color = self.compile_expr(runtime_ptr, color)?;
                self.call_void(
                    self.runtime.pen_set_color,
                    &[runtime_ptr.into(), color.into()],
                )?;
            }
            Stmt::PenStamp => {
                self.call_void(self.runtime.pen_stamp, &[runtime_ptr.into()])?;
            }
            Stmt::PenSetColorParam { param, value } => {
                let value = self.compile_expr(runtime_ptr, value)?;
                let param_code = self
                    .i64_type
                    .const_int(Self::pen_color_param_code(*param), false);
                self.call_void(
                    self.runtime.pen_set_color_param,
                    &[runtime_ptr.into(), param_code.into(), value.into()],
                )?;
            }
            Stmt::EventBroadcast { message, wait } => {
                let message_value = self.compile_expr(runtime_ptr, message)?;
                if *wait {
                    self.call_void(
                        self.runtime.event_broadcast_wait_value,
                        &[runtime_ptr.into(), message_value.into()],
                    )?;
                } else {
                    self.call_void(
                        self.runtime.event_broadcast_value,
                        &[runtime_ptr.into(), message_value.into()],
                    )?;
                }
            }
            Stmt::ProcedureCall {
                procedure_index,
                args,
            } => {
                let function = self
                    .procedure_functions
                    .get(procedure_index)
                    .copied()
                    .ok_or_else(|| {
                        anyhow!("missing procedure function index {}", procedure_index)
                    })?;
                let is_warp = self
                    .procedure_warp_flags
                    .get(procedure_index)
                    .copied()
                    .unwrap_or(false);
                let call_args = self.build_procedure_call_args(runtime_ptr, function, args)?;
                if is_warp {
                    self.call_void(self.runtime.warp_enter, &[runtime_ptr.into()])?;
                }
                self.build(self.builder.build_call(function, &call_args, ""))?;
                if is_warp {
                    self.call_void(self.runtime.warp_leave, &[runtime_ptr.into()])?;
                }
            }
            Stmt::ProcedureReturn { value } => {
                let value = self.compile_expr(runtime_ptr, value)?;
                self.build(self.builder.build_return(Some(&value)))?;
            }
        }
        Ok(())
    }

    fn build_procedure_call_args(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        function: FunctionValue<'ctx>,
        args: &[Expr],
    ) -> Result<Vec<BasicMetadataValueEnum<'ctx>>> {
        let arg_count = function.count_params() as usize;
        let mut call_args = Vec::with_capacity(arg_count.max(1));
        call_args.push(runtime_ptr.into());

        for arg_index in 0..(arg_count.saturating_sub(1)) {
            if let Some(expr) = args.get(arg_index) {
                call_args.push(self.compile_expr(runtime_ptr, expr)?.into());
            } else {
                call_args.push(self.f64_type.const_zero().into());
            }
        }

        Ok(call_args)
    }

    fn compile_repeat(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        times: &Expr,
        body: &[Stmt],
    ) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("loop emitted without active function"))?;

        let raw_count = self.compile_expr(runtime_ptr, times)?;
        let count = self.call_i64(
            self.runtime.repeat_count,
            &[runtime_ptr.into(), raw_count.into()],
            "repeat.count",
        )?;

        let index_ptr = self.build(self.builder.build_alloca(self.i64_type, "repeat.index"))?;
        self.build(
            self.builder
                .build_store(index_ptr, self.i64_type.const_zero()),
        )?;

        let guard_counter_ptr = self.alloc_loop_guard_counter("repeat")?;
        let cond_block = self.context.append_basic_block(function, "repeat.cond");
        let body_block = self.context.append_basic_block(function, "repeat.body");
        let end_block = self.context.append_basic_block(function, "repeat.end");

        self.build(self.builder.build_unconditional_branch(cond_block))?;

        self.builder.position_at_end(cond_block);
        let index = self
            .build(
                self.builder
                    .build_load(self.i64_type, index_ptr, "repeat.index.load"),
            )?
            .into_int_value();
        let has_iterations = self.build(self.builder.build_int_compare(
            IntPredicate::ULT,
            index,
            count,
            "repeat.has_iterations",
        ))?;
        let loop_guard_fn = self.current_loop_guard();
        let guard =
            self.emit_inline_guard(runtime_ptr, guard_counter_ptr, loop_guard_fn, "repeat")?;
        let continue_loop = self.build(self.builder.build_and(
            has_iterations,
            guard,
            "repeat.continue",
        ))?;
        self.build(
            self.builder
                .build_conditional_branch(continue_loop, body_block, end_block),
        )?;

        self.builder.position_at_end(body_block);
        for statement in body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            let next = self.build(self.builder.build_int_add(
                index,
                self.i64_type.const_int(1, false),
                "repeat.next",
            ))?;
            self.build(self.builder.build_store(index_ptr, next))?;
            self.build(self.builder.build_unconditional_branch(cond_block))?;
        }

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_for_each(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        variable_index: usize,
        count_expr: &Expr,
        body: &[Stmt],
    ) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("for-each loop emitted without active function"))?;

        let raw_count = self.compile_expr(runtime_ptr, count_expr)?;
        let count = self.call_i64(
            self.runtime.repeat_count,
            &[runtime_ptr.into(), raw_count.into()],
            "for_each.count",
        )?;

        let index_ptr = self.build(self.builder.build_alloca(self.i64_type, "for_each.index"))?;
        self.build(
            self.builder
                .build_store(index_ptr, self.i64_type.const_zero()),
        )?;

        let guard_counter_ptr = self.alloc_loop_guard_counter("for_each")?;
        let cond_block = self.context.append_basic_block(function, "for_each.cond");
        let body_block = self.context.append_basic_block(function, "for_each.body");
        let end_block = self.context.append_basic_block(function, "for_each.end");

        self.build(self.builder.build_unconditional_branch(cond_block))?;

        self.builder.position_at_end(cond_block);
        let index = self
            .build(
                self.builder
                    .build_load(self.i64_type, index_ptr, "for_each.index.load"),
            )?
            .into_int_value();
        let has_iterations = self.build(self.builder.build_int_compare(
            IntPredicate::ULT,
            index,
            count,
            "for_each.has_iterations",
        ))?;
        let loop_guard_fn = self.current_loop_guard();
        let guard =
            self.emit_inline_guard(runtime_ptr, guard_counter_ptr, loop_guard_fn, "for_each")?;
        let should_continue = self.build(self.builder.build_and(
            has_iterations,
            guard,
            "for_each.continue",
        ))?;
        self.build(
            self.builder
                .build_conditional_branch(should_continue, body_block, end_block),
        )?;

        self.builder.position_at_end(body_block);
        let next_index = self.build(self.builder.build_int_add(
            index,
            self.i64_type.const_int(1, false),
            "for_each.next",
        ))?;
        let next_index_f = self.build(self.builder.build_unsigned_int_to_float(
            next_index,
            self.f64_type,
            "for_each.next.f64",
        ))?;
        self.build(self.builder.build_store(index_ptr, next_index))?;
        self.call_void(
            self.runtime.set_var,
            &[
                runtime_ptr.into(),
                self.i64_type.const_int(variable_index as u64, false).into(),
                next_index_f.into(),
            ],
        )?;

        for statement in body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_unconditional_branch(cond_block))?;
        }

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_forever(&mut self, runtime_ptr: PointerValue<'ctx>, body: &[Stmt]) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("forever loop emitted without active function"))?;

        let guard_counter_ptr = self.alloc_loop_guard_counter("forever")?;
        let cond_block = self.context.append_basic_block(function, "forever.cond");
        let body_block = self.context.append_basic_block(function, "forever.body");
        let end_block = self.context.append_basic_block(function, "forever.end");

        self.build(self.builder.build_unconditional_branch(cond_block))?;

        self.builder.position_at_end(cond_block);
        let loop_guard_fn = self.current_forever_guard();
        let should_continue =
            self.emit_inline_guard(runtime_ptr, guard_counter_ptr, loop_guard_fn, "forever")?;
        self.build(
            self.builder
                .build_conditional_branch(should_continue, body_block, end_block),
        )?;

        self.builder.position_at_end(body_block);
        for statement in body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_unconditional_branch(cond_block))?;
        }

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_while(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        condition: &Expr,
        body: &[Stmt],
    ) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("while loop emitted without active function"))?;

        let guard_counter_ptr = self.alloc_loop_guard_counter("while")?;
        let cond_block = self.context.append_basic_block(function, "while.cond");
        let body_block = self.context.append_basic_block(function, "while.body");
        let end_block = self.context.append_basic_block(function, "while.end");

        self.build(self.builder.build_unconditional_branch(cond_block))?;

        self.builder.position_at_end(cond_block);
        let loop_guard_fn = self.current_loop_guard();
        let guard =
            self.emit_inline_guard(runtime_ptr, guard_counter_ptr, loop_guard_fn, "while")?;
        let condition = self.compile_condition(runtime_ptr, condition)?;
        let should_continue = self.build(self.builder.build_and(guard, condition, "while.cont"))?;
        self.build(
            self.builder
                .build_conditional_branch(should_continue, body_block, end_block),
        )?;

        self.builder.position_at_end(body_block);
        for statement in body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_unconditional_branch(cond_block))?;
        }

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_repeat_until(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        condition: &Expr,
        body: &[Stmt],
    ) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("repeat-until loop emitted without active function"))?;

        let guard_counter_ptr = self.alloc_loop_guard_counter("repeat_until")?;
        let cond_block = self
            .context
            .append_basic_block(function, "repeat_until.cond");
        let body_block = self
            .context
            .append_basic_block(function, "repeat_until.body");
        let end_block = self
            .context
            .append_basic_block(function, "repeat_until.end");

        self.build(self.builder.build_unconditional_branch(cond_block))?;

        self.builder.position_at_end(cond_block);
        let loop_guard_fn = self.current_loop_guard();
        let guard = self.emit_inline_guard(
            runtime_ptr,
            guard_counter_ptr,
            loop_guard_fn,
            "repeat_until",
        )?;
        let done = self.compile_condition(runtime_ptr, condition)?;
        let continue_loop = self.build(self.builder.build_not(done, "repeat_until.not_done"))?;
        let should_continue = self.build(self.builder.build_and(
            guard,
            continue_loop,
            "repeat_until.cont",
        ))?;
        self.build(
            self.builder
                .build_conditional_branch(should_continue, body_block, end_block),
        )?;

        self.builder.position_at_end(body_block);
        for statement in body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_unconditional_branch(cond_block))?;
        }

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_wait_until(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        condition: &Expr,
    ) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("wait-until emitted without active function"))?;

        let guard_counter_ptr = self.alloc_loop_guard_counter("wait_until")?;
        let cond_block = self.context.append_basic_block(function, "wait_until.cond");
        let end_block = self.context.append_basic_block(function, "wait_until.end");

        self.build(self.builder.build_unconditional_branch(cond_block))?;

        self.builder.position_at_end(cond_block);
        let loop_guard_fn = self.current_forever_guard();
        let guard =
            self.emit_inline_guard(runtime_ptr, guard_counter_ptr, loop_guard_fn, "wait_until")?;
        let done = self.compile_condition(runtime_ptr, condition)?;
        let wait_more = self.build(self.builder.build_not(done, "wait_until.not_done"))?;
        let should_continue =
            self.build(self.builder.build_and(guard, wait_more, "wait_until.cont"))?;
        self.build(
            self.builder
                .build_conditional_branch(should_continue, cond_block, end_block),
        )?;

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_if(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        condition: &Expr,
        then_body: &[Stmt],
        else_body: &[Stmt],
    ) -> Result<()> {
        let function = self
            .current_fn
            .ok_or_else(|| anyhow!("if emitted without active function"))?;

        let condition = self.compile_condition(runtime_ptr, condition)?;
        let then_block = self.context.append_basic_block(function, "if.then");
        let else_block = self.context.append_basic_block(function, "if.else");
        let end_block = self.context.append_basic_block(function, "if.end");

        self.build(
            self.builder
                .build_conditional_branch(condition, then_block, else_block),
        )?;

        self.builder.position_at_end(then_block);
        for statement in then_body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_unconditional_branch(end_block))?;
        }

        self.builder.position_at_end(else_block);
        for statement in else_body {
            self.compile_statement(runtime_ptr, statement)?;
            if self
                .builder
                .get_insert_block()
                .and_then(|bb| bb.get_terminator())
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_none()
        {
            self.build(self.builder.build_unconditional_branch(end_block))?;
        }

        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn compile_condition(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        condition: &Expr,
    ) -> Result<IntValue<'ctx>> {
        let value = self.compile_expr(runtime_ptr, condition)?;
        self.build(self.builder.build_float_compare(
            FloatPredicate::ONE,
            value,
            self.f64_type.const_zero(),
            "if.cond",
        ))
    }

    fn compile_expr(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        expr: &Expr,
    ) -> Result<FloatValue<'ctx>> {
        match expr {
            Expr::Number(number) => Ok(self.f64_type.const_float(*number)),
            Expr::StringLiteral(index) => {
                let bits = encode_string_id(*index).to_bits();
                let raw = self.i64_type.const_int(bits, false);
                let value = self.build(self.builder.build_bit_cast(
                    raw,
                    self.f64_type,
                    "string.literal",
                ))?;
                Ok(value.into_float_value())
            }
            Expr::MotionXPosition => self.call_f64(
                self.runtime.motion_x_position,
                &[runtime_ptr.into()],
                "motion.x_position",
            ),
            Expr::MotionYPosition => self.call_f64(
                self.runtime.motion_y_position,
                &[runtime_ptr.into()],
                "motion.y_position",
            ),
            Expr::SensingMouseX => self.call_f64(
                self.runtime.sensing_mouse_x,
                &[runtime_ptr.into()],
                "sensing.mouse_x",
            ),
            Expr::SensingMouseY => self.call_f64(
                self.runtime.sensing_mouse_y,
                &[runtime_ptr.into()],
                "sensing.mouse_y",
            ),
            Expr::SensingMouseDown => self.call_f64(
                self.runtime.sensing_mouse_down,
                &[runtime_ptr.into()],
                "sensing.mouse_down",
            ),
            Expr::LooksCostumeNumber => self.call_f64(
                self.runtime.looks_costume_number,
                &[runtime_ptr.into()],
                "looks.costume_number",
            ),
            Expr::LooksCostumeName => self.call_f64(
                self.runtime.looks_costume_name,
                &[runtime_ptr.into()],
                "looks.costume_name",
            ),
            Expr::SensingOf { object, property } => {
                let object = self.compile_expr(runtime_ptr, object)?;
                let bits = encode_string_id(*property).to_bits();
                let property_raw = self.i64_type.const_int(bits, false);
                let property = self.build(self.builder.build_bit_cast(
                    property_raw,
                    self.f64_type,
                    "sensing.of.property",
                ))?;
                self.call_f64(
                    self.runtime.sensing_of,
                    &[runtime_ptr.into(), object.into(), property.into()],
                    "sensing.of",
                )
            }
            Expr::SensingCurrent(menu) => {
                let menu_code = self
                    .i64_type
                    .const_int(Self::sensing_current_menu_code(*menu), false);
                self.call_f64(
                    self.runtime.sensing_current,
                    &[runtime_ptr.into(), menu_code.into()],
                    "sensing.current",
                )
            }
            Expr::Variable(index) => {
                if let Some(vars_ptr) = self.cached_vars_ptr {
                    // Inline variable load: GEP into cached variables array
                    let gep = unsafe {
                        self.build(self.builder.build_in_bounds_gep(
                            self.f64_type,
                            vars_ptr,
                            &[self.i64_type.const_int(*index as u64, false)],
                            "var.gep",
                        ))?
                    };
                    let value = self
                        .build(self.builder.build_load(self.f64_type, gep, "var.get"))?
                        .into_float_value();
                    Ok(value)
                } else {
                    // Fallback: use extern call
                    let index_value = self.i64_type.const_int(*index as u64, false);
                    self.call_f64(
                        self.runtime.get_var,
                        &[runtime_ptr.into(), index_value.into()],
                        "var.get",
                    )
                }
            }
            Expr::ProcedureArg(index) => Ok(self
                .current_proc_params
                .get(*index)
                .copied()
                .unwrap_or_else(|| self.f64_type.const_zero())),
            Expr::ProcedureCall {
                procedure_index,
                args,
            } => {
                let function = self
                    .procedure_functions
                    .get(procedure_index)
                    .copied()
                    .ok_or_else(|| {
                        anyhow!("missing procedure function index {}", procedure_index)
                    })?;
                let is_warp = self
                    .procedure_warp_flags
                    .get(procedure_index)
                    .copied()
                    .unwrap_or(false);
                let call_args = self.build_procedure_call_args(runtime_ptr, function, args)?;
                if is_warp {
                    self.call_void(self.runtime.warp_enter, &[runtime_ptr.into()])?;
                }
                let result = self.call_f64(function, &call_args, "proc.call")?;
                if is_warp {
                    self.call_void(self.runtime.warp_leave, &[runtime_ptr.into()])?;
                }
                Ok(result)
            }
            Expr::Add(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_binop(
                    runtime_ptr,
                    left,
                    right,
                    "add",
                    |s, l, r| s.build(s.builder.build_float_add(l, r, "add.fast")),
                    self.runtime.operator_add,
                )
            }
            Expr::Subtract(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_binop(
                    runtime_ptr,
                    left,
                    right,
                    "sub",
                    |s, l, r| s.build(s.builder.build_float_sub(l, r, "sub.fast")),
                    self.runtime.operator_subtract,
                )
            }
            Expr::Multiply(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_binop(
                    runtime_ptr,
                    left,
                    right,
                    "mul",
                    |s, l, r| s.build(s.builder.build_float_mul(l, r, "mul.fast")),
                    self.runtime.operator_multiply,
                )
            }
            Expr::Divide(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_binop(
                    runtime_ptr,
                    left,
                    right,
                    "div",
                    |s, l, r| s.build(s.builder.build_float_div(l, r, "div.fast")),
                    self.runtime.operator_divide,
                )
            }
            Expr::Mod(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                // Mod has Scratch-specific floored semantics; use extern for
                // both fast and slow paths to keep correctness simple.
                self.call_f64(
                    self.runtime.operator_mod,
                    &[runtime_ptr.into(), left.into(), right.into()],
                    "mod",
                )
            }
            Expr::GreaterThan(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_cmp(
                    runtime_ptr,
                    left,
                    right,
                    "gt",
                    FloatPredicate::OGT,
                    self.runtime.operator_greater_than,
                )
            }
            Expr::LessThan(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_cmp(
                    runtime_ptr,
                    left,
                    right,
                    "lt",
                    FloatPredicate::OLT,
                    self.runtime.operator_less_than,
                )
            }
            Expr::Equals(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.compile_inline_cmp(
                    runtime_ptr,
                    left,
                    right,
                    "eq",
                    FloatPredicate::OEQ,
                    self.runtime.operator_equals,
                )
            }
            Expr::Random(from, to) => {
                let from = self.compile_expr(runtime_ptr, from)?;
                let to = self.compile_expr(runtime_ptr, to)?;
                self.call_f64(
                    self.runtime.random,
                    &[runtime_ptr.into(), from.into(), to.into()],
                    "rand",
                )
            }
            Expr::And(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                let left_bool = self.compile_truthy(left)?;
                let right_bool = self.compile_truthy(right)?;
                let and_value = self.build(self.builder.build_and(left_bool, right_bool, "and"))?;
                self.build(self.builder.build_unsigned_int_to_float(
                    and_value,
                    self.f64_type,
                    "and.float",
                ))
            }
            Expr::Or(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                let left_bool = self.compile_truthy(left)?;
                let right_bool = self.compile_truthy(right)?;
                let or_value = self.build(self.builder.build_or(left_bool, right_bool, "or"))?;
                self.build(self.builder.build_unsigned_int_to_float(
                    or_value,
                    self.f64_type,
                    "or.float",
                ))
            }
            Expr::Not(inner) => {
                let value = self.compile_expr(runtime_ptr, inner)?;
                let bool_value = self.compile_truthy(value)?;
                let not_value = self.build(self.builder.build_not(bool_value, "not"))?;
                self.build(self.builder.build_unsigned_int_to_float(
                    not_value,
                    self.f64_type,
                    "not.float",
                ))
            }
            Expr::StringLength(inner) => {
                let value = self.compile_expr(runtime_ptr, inner)?;
                self.call_f64(
                    self.runtime.operator_length,
                    &[runtime_ptr.into(), value.into()],
                    "length",
                )
            }
            Expr::StringJoin(left, right) => {
                let left = self.compile_expr(runtime_ptr, left)?;
                let right = self.compile_expr(runtime_ptr, right)?;
                self.call_f64(
                    self.runtime.operator_join,
                    &[runtime_ptr.into(), left.into(), right.into()],
                    "string.join",
                )
            }
            Expr::StringContains(text, part) => {
                let text = self.compile_expr(runtime_ptr, text)?;
                let part = self.compile_expr(runtime_ptr, part)?;
                self.call_f64(
                    self.runtime.operator_contains,
                    &[runtime_ptr.into(), text.into(), part.into()],
                    "string.contains",
                )
            }
            Expr::Round(inner) => {
                let value = self.compile_expr(runtime_ptr, inner)?;
                self.call_f64(
                    self.runtime.operator_round,
                    &[runtime_ptr.into(), value.into()],
                    "round",
                )
            }
            Expr::LetterOf { letter, string } => {
                let letter = self.compile_expr(runtime_ptr, letter)?;
                let string = self.compile_expr(runtime_ptr, string)?;
                self.call_f64(
                    self.runtime.operator_letter_of,
                    &[runtime_ptr.into(), letter.into(), string.into()],
                    "letter.of",
                )
            }
            Expr::ListItem { list_index, index } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                let index = self.compile_expr(runtime_ptr, index)?;
                self.call_f64(
                    self.runtime.data_item_of_list,
                    &[runtime_ptr.into(), list_index.into(), index.into()],
                    "list.item",
                )
            }
            Expr::ListItemNum { list_index, item } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                let item = self.compile_expr(runtime_ptr, item)?;
                self.call_f64(
                    self.runtime.data_item_num_of_list,
                    &[runtime_ptr.into(), list_index.into(), item.into()],
                    "list.itemnum",
                )
            }
            Expr::ListLength { list_index } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                self.call_f64(
                    self.runtime.data_length_of_list,
                    &[runtime_ptr.into(), list_index.into()],
                    "list.length",
                )
            }
            Expr::ListContainsItem { list_index, item } => {
                let list_index = self.i64_type.const_int(*list_index as u64, false);
                let item = self.compile_expr(runtime_ptr, item)?;
                self.call_f64(
                    self.runtime.data_list_contains_item,
                    &[runtime_ptr.into(), list_index.into(), item.into()],
                    "list.contains",
                )
            }
            Expr::KeyPressed(key) => {
                let key = self.compile_expr(runtime_ptr, key)?;
                self.call_f64(
                    self.runtime.sensing_key_pressed,
                    &[runtime_ptr.into(), key.into()],
                    "sensing.key_pressed",
                )
            }
            Expr::SensingAnswer => self.call_f64(
                self.runtime.sensing_answer,
                &[runtime_ptr.into()],
                "sensing.answer",
            ),
            Expr::SensingTimer => self.call_f64(
                self.runtime.sensing_timer,
                &[runtime_ptr.into()],
                "sensing.timer",
            ),
            Expr::SensingDaysSince2000 => self.call_f64(
                self.runtime.sensing_days_since_2000,
                &[runtime_ptr.into()],
                "sensing.days_since_2000",
            ),
            Expr::SensingTouchingObject(object) => {
                let object = self.compile_expr(runtime_ptr, object)?;
                self.call_f64(
                    self.runtime.sensing_touching_object,
                    &[runtime_ptr.into(), object.into()],
                    "sensing.touching_object",
                )
            }
            Expr::SensingTouchingColor(color) => {
                let color = self.compile_expr(runtime_ptr, color)?;
                self.call_f64(
                    self.runtime.sensing_touching_color,
                    &[runtime_ptr.into(), color.into()],
                    "sensing.touching_color",
                )
            }
            Expr::MathOp { op, value } => {
                let code = self.f64_type.const_float(Self::mathop_code(op) as f64);
                let value = self.compile_expr(runtime_ptr, value)?;
                self.call_f64(
                    self.runtime.operator_mathop,
                    &[runtime_ptr.into(), code.into(), value.into()],
                    "mathop",
                )
            }
        }
    }

    /// Emit an inline fast-path for a binary arithmetic operation.
    ///
    /// When both operands are plain IEEE-754 numbers (not NaN-boxed strings),
    /// the fast path executes the native LLVM instruction directly (fadd, fsub,
    /// fmul, fdiv).  Only when at least one operand carries the string tag
    /// do we fall back to the extern runtime function for string→number
    /// coercion.  This eliminates the function-call overhead for the
    /// overwhelmingly common all-numeric case.
    fn compile_inline_binop(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        left: FloatValue<'ctx>,
        right: FloatValue<'ctx>,
        label: &str,
        fast_op: impl FnOnce(&Self, FloatValue<'ctx>, FloatValue<'ctx>) -> Result<FloatValue<'ctx>>,
        slow_fn: FunctionValue<'ctx>,
    ) -> Result<FloatValue<'ctx>> {
        let function = self.current_fn.unwrap();
        let fast_bb = self
            .context
            .append_basic_block(function, &format!("{label}.fast"));
        let slow_bb = self
            .context
            .append_basic_block(function, &format!("{label}.slow"));
        let merge_bb = self
            .context
            .append_basic_block(function, &format!("{label}.merge"));

        let either_is_string = self.compile_either_is_string(left, right, label)?;
        self.build(
            self.builder
                .build_conditional_branch(either_is_string, slow_bb, fast_bb),
        )?;

        // Fast path: native LLVM arithmetic
        self.builder.position_at_end(fast_bb);
        let fast_result = fast_op(self, left, right)?;
        let fast_exit_bb = self.builder.get_insert_block().unwrap();
        self.build(self.builder.build_unconditional_branch(merge_bb))?;

        // Slow path: call extern with string→number coercion
        self.builder.position_at_end(slow_bb);
        let slow_result = self.call_f64(
            slow_fn,
            &[runtime_ptr.into(), left.into(), right.into()],
            &format!("{label}.slow.result"),
        )?;
        let slow_exit_bb = self.builder.get_insert_block().unwrap();
        self.build(self.builder.build_unconditional_branch(merge_bb))?;

        // Merge
        self.builder.position_at_end(merge_bb);
        let phi = self.build(
            self.builder
                .build_phi(self.f64_type, &format!("{label}.result")),
        )?;
        phi.add_incoming(&[(&fast_result, fast_exit_bb), (&slow_result, slow_exit_bb)]);
        Ok(phi.as_basic_value().into_float_value())
    }

    /// Emit an inline fast-path for a comparison operation.
    ///
    /// When both operands are plain numbers, uses native LLVM fcmp.
    /// Falls back to the extern function when strings are involved.
    fn compile_inline_cmp(
        &mut self,
        runtime_ptr: PointerValue<'ctx>,
        left: FloatValue<'ctx>,
        right: FloatValue<'ctx>,
        label: &str,
        predicate: FloatPredicate,
        slow_fn: FunctionValue<'ctx>,
    ) -> Result<FloatValue<'ctx>> {
        let function = self.current_fn.unwrap();
        let fast_bb = self
            .context
            .append_basic_block(function, &format!("{label}.fast"));
        let slow_bb = self
            .context
            .append_basic_block(function, &format!("{label}.slow"));
        let merge_bb = self
            .context
            .append_basic_block(function, &format!("{label}.merge"));

        let either_is_string = self.compile_either_is_string(left, right, label)?;
        self.build(
            self.builder
                .build_conditional_branch(either_is_string, slow_bb, fast_bb),
        )?;

        // Fast path: native comparison → 1.0 or 0.0
        self.builder.position_at_end(fast_bb);
        let cmp_result = self.build(self.builder.build_float_compare(
            predicate,
            left,
            right,
            &format!("{label}.cmp"),
        ))?;
        let fast_result = self.build(self.builder.build_unsigned_int_to_float(
            cmp_result,
            self.f64_type,
            &format!("{label}.fast.result"),
        ))?;
        let fast_exit_bb = self.builder.get_insert_block().unwrap();
        self.build(self.builder.build_unconditional_branch(merge_bb))?;

        // Slow path
        self.builder.position_at_end(slow_bb);
        let slow_result = self.call_f64(
            slow_fn,
            &[runtime_ptr.into(), left.into(), right.into()],
            &format!("{label}.slow.result"),
        )?;
        let slow_exit_bb = self.builder.get_insert_block().unwrap();
        self.build(self.builder.build_unconditional_branch(merge_bb))?;

        // Merge
        self.builder.position_at_end(merge_bb);
        let phi = self.build(
            self.builder
                .build_phi(self.f64_type, &format!("{label}.result")),
        )?;
        phi.add_incoming(&[(&fast_result, fast_exit_bb), (&slow_result, slow_exit_bb)]);
        Ok(phi.as_basic_value().into_float_value())
    }

    /// Check whether either of two f64 values carries the NaN-boxed string
    /// tag.  Returns an i1 that is true when at least one is a string.
    fn compile_either_is_string(
        &self,
        left: FloatValue<'ctx>,
        right: FloatValue<'ctx>,
        label: &str,
    ) -> Result<IntValue<'ctx>> {
        let mask = self.i64_type.const_int(STRING_TAG_MASK, false);
        let tag = self.i64_type.const_int(STRING_TAG_BITS, false);

        let left_bits = self
            .build(
                self.builder
                    .build_bit_cast(left, self.i64_type, &format!("{label}.l.bits")),
            )?
            .into_int_value();
        let right_bits = self
            .build(
                self.builder
                    .build_bit_cast(right, self.i64_type, &format!("{label}.r.bits")),
            )?
            .into_int_value();

        // Check each operand individually — ORing raw bits before masking
        // would cause false positives when two non-string values happen to
        // OR into the string tag pattern.
        let left_masked = self.build(self.builder.build_and(
            left_bits,
            mask,
            &format!("{label}.l.masked"),
        ))?;
        let left_is_str = self.build(self.builder.build_int_compare(
            IntPredicate::EQ,
            left_masked,
            tag,
            &format!("{label}.l.is_str"),
        ))?;

        let right_masked = self.build(self.builder.build_and(
            right_bits,
            mask,
            &format!("{label}.r.masked"),
        ))?;
        let right_is_str = self.build(self.builder.build_int_compare(
            IntPredicate::EQ,
            right_masked,
            tag,
            &format!("{label}.r.is_str"),
        ))?;

        self.build(
            self.builder
                .build_or(left_is_str, right_is_str, &format!("{label}.either_str")),
        )
    }

    fn compile_truthy(&self, value: FloatValue<'ctx>) -> Result<IntValue<'ctx>> {
        self.build(self.builder.build_float_compare(
            FloatPredicate::ONE,
            value,
            self.f64_type.const_zero(),
            "truthy",
        ))
    }

    fn build_return_for_current_function(&self) -> Result<()> {
        let Some(function) = self.current_fn else {
            return Err(anyhow!("return emitted without active function"));
        };
        if function.get_type().get_return_type().is_some() {
            self.build(self.builder.build_return(Some(&self.f64_type.const_zero())))?;
        } else {
            self.build(self.builder.build_return(None))?;
        }
        Ok(())
    }

    fn call_void(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
    ) -> Result<()> {
        self.build(self.builder.build_call(function, args, ""))
            .map(|_| ())
    }

    fn call_f64(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<FloatValue<'ctx>> {
        let call = self.build(self.builder.build_call(function, args, name))?;
        let value = call.try_as_basic_value().basic().ok_or_else(|| {
            anyhow!(
                "expected non-void return from {}",
                function.get_name().to_string_lossy()
            )
        })?;
        Ok(value.into_float_value())
    }

    fn call_i64(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<IntValue<'ctx>> {
        let call = self.build(self.builder.build_call(function, args, name))?;
        let value = call.try_as_basic_value().basic().ok_or_else(|| {
            anyhow!(
                "expected non-void return from {}",
                function.get_name().to_string_lossy()
            )
        })?;
        Ok(value.into_int_value())
    }

    fn call_i1(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<IntValue<'ctx>> {
        let call = self.build(self.builder.build_call(function, args, name))?;
        let value = call.try_as_basic_value().basic().ok_or_else(|| {
            anyhow!(
                "expected non-void return from {}",
                function.get_name().to_string_lossy()
            )
        })?;
        Ok(value.into_int_value())
    }

    fn call_ptr(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<PointerValue<'ctx>> {
        let call = self.build(self.builder.build_call(function, args, name))?;
        let value = call.try_as_basic_value().basic().ok_or_else(|| {
            anyhow!(
                "expected non-void return from {}",
                function.get_name().to_string_lossy()
            )
        })?;
        Ok(value.into_pointer_value())
    }

    fn build<T>(&self, result: std::result::Result<T, BuilderError>) -> Result<T> {
        result.map_err(|error| anyhow!("LLVM builder error: {}", error))
    }

    fn mathop_code(op: &MathOp) -> u64 {
        match op {
            MathOp::Abs => 0,
            MathOp::Floor => 1,
            MathOp::Ceil => 2,
            MathOp::Sqrt => 3,
            MathOp::Sin => 4,
            MathOp::Cos => 5,
            MathOp::Tan => 6,
            MathOp::Asin => 7,
            MathOp::Acos => 8,
            MathOp::Atan => 9,
            MathOp::Ln => 10,
            MathOp::Log => 11,
            MathOp::Exp => 12,
            MathOp::Exp10 => 13,
        }
    }

    fn stop_mode_code(mode: ControlStopMode) -> u64 {
        match mode {
            ControlStopMode::ThisScript => 0,
            ControlStopMode::All => 1,
            ControlStopMode::OtherScriptsInTarget => 2,
        }
    }

    fn sensing_current_menu_code(menu: SensingCurrentMenu) -> u64 {
        match menu {
            SensingCurrentMenu::Year => 0,
            SensingCurrentMenu::Month => 1,
            SensingCurrentMenu::Date => 2,
            SensingCurrentMenu::DayOfWeek => 3,
            SensingCurrentMenu::Hour => 4,
            SensingCurrentMenu::Minute => 5,
            SensingCurrentMenu::Second => 6,
        }
    }

    fn pen_color_param_code(param: PenColorParam) -> u64 {
        match param {
            PenColorParam::Color => 0,
            PenColorParam::Saturation => 1,
            PenColorParam::Brightness => 2,
            PenColorParam::Transparency => 3,
        }
    }
}
