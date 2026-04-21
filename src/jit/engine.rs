use inkwell::{
    OptimizationLevel,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassBuilderOptions,
    support::load_visible_symbols,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
};

use crate::{
    compiler::types::Builders,
    jit::{memory::SectionMemoryManager, runtime},
};

type ThreadThunk = unsafe extern "C" fn(*mut runtime::SpriteState);

pub fn run<'ctx>(builders: &Builders<'ctx>, thread_functions: &[String]) {
    let target_machine = prepare_module(&builders.module);
    optimize_module(&builders.module, &target_machine);
    builders.module.verify().unwrap_or_else(|err| {
        panic!(
            "generated module is invalid:\n{err}\n{}",
            builders.module.to_string()
        )
    });

    load_visible_symbols();
    ExecutionEngine::link_in_mc_jit();
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

    for function_name in thread_functions {
        let mut state = runtime::SpriteState::default();
        let function: JitFunction<'_, ThreadThunk> =
            unsafe { execution_engine.get_function(function_name) }
                .unwrap_or_else(|err| panic!("failed to find JIT function {function_name}: {err}"));

        unsafe {
            function.call(&mut state);
        }

        println!("{:?}", state);
    }
}

fn prepare_module<'ctx>(module: &Module<'ctx>) -> TargetMachine {
    Target::initialize_native(&InitializationConfig::default())
        .expect("failed to initialize native LLVM target");

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple)
        .unwrap_or_else(|err| panic!("failed to resolve target from triple: {err}"));
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
