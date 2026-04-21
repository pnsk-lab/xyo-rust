use inkwell::{
    AddressSpace, OptimizationLevel,
    context::Context,
    execution_engine::JitFunction,
    passes::PassBuilderOptions,
    support::{load_library_permanently, load_visible_symbols},
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};
use std::path::Path;

use crate::{
    compiler::{
        blocks::{
            literal::parse_literal_expr, motion::parse_motion_stmt, operator::parse_operator_expr,
        },
        types::Builders,
    },
    parser::types::{Expr, Stmt, Thread},
    types::ScratchProject,
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct JitSpriteState {
    sprite_x: f64,
    sprite_y: f64,
    sprite_rotate: f64,
}

type ThreadThunk = unsafe extern "C" fn(*mut JitSpriteState);

pub fn compiler(project: &ScratchProject, threads: &Vec<Thread>) {
    Target::initialize_native(&InitializationConfig::default())
        .expect("failed to initialize native LLVM target");
    let context = Context::create();
    let mut builders = Builders::new(&context, project);
    let mut thread_functions = Vec::with_capacity(threads.len());

    threads.iter().for_each(|v| {
        thread_functions.push(generate_thread_ir(&mut builders, v));
    });

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &TargetMachine::get_host_cpu_name().to_string(),
            &TargetMachine::get_host_cpu_features().to_string(),
            OptimizationLevel::Aggressive,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .unwrap();

    builders.module.set_triple(&target_triple);
    builders
        .module
        .set_data_layout(&target_machine.get_target_data().get_data_layout());

    let passes = "default<O3>";
    let pass_builder_options = PassBuilderOptions::create();
    pass_builder_options.set_loop_interleaving(true);
    pass_builder_options.set_loop_vectorization(true);
    pass_builder_options.set_loop_slp_vectorization(true);
    pass_builder_options.set_loop_unrolling(true);
    pass_builder_options.set_forget_all_scev_in_loop_unroll(true);
    builders
        .module
        .run_passes(passes, &target_machine, pass_builder_options)
        .unwrap();
    builders.module.verify().unwrap_or_else(|err| {
        panic!(
            "generated module is invalid:\n{err}\n{}",
            builders.module.to_string()
        )
    });
    run_threads_with_jit(&builders, &thread_functions);
}

fn run_threads_with_jit(builders: &Builders<'_>, thread_functions: &[String]) {
    let execution_engine = builders
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    for function_name in thread_functions.iter() {
        let mut state = JitSpriteState::default();
        let function: JitFunction<'_, ThreadThunk> =
            unsafe { execution_engine.get_function(function_name) }
                .unwrap_or_else(|err| panic!("failed to find JIT function {function_name}: {err}"));

        unsafe {
            function.call(&mut state);
        }

        println!("{:?}", state)
    }
}

pub fn generate_thread_ir(builders: &mut Builders, thread: &Thread) -> String {
    let ptr_type = builders.context.ptr_type(AddressSpace::default());
    let fn_type = builders
        .context
        .void_type()
        .fn_type(&[ptr_type.into()], false);
    let function_name = builders.create_function_name();
    let function = builders.module.add_function(&function_name, fn_type, None);
    let entry = builders.context.append_basic_block(function, "entry");
    builders.builder.position_at_end(entry);
    for block in &thread.stmts {
        match block {
            Stmt::Motion(v) => parse_motion_stmt(builders, v, &function, thread.target_idx),
            _ => todo!("やります"),
        }
    }
    builders.builder.build_return(None).unwrap();
    function_name
}

pub fn generate_expr_ir<'ctx>(
    builders: &Builders<'ctx>,
    expr: &Expr,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        Expr::Literal(l) => parse_literal_expr(builders, l, function, target_idx),
        Expr::Operator(l) => parse_operator_expr(builders, l, function, target_idx),
        _ => todo!("やる"),
    }
}

pub enum ScratchReturnTypes<'ctx> {
    Number(FloatValue<'ctx>),
    String(PointerValue<'ctx>),
    Bool(IntValue<'ctx>),
    NumberLiteral(f64),
    StringLiteral((String, PointerValue<'ctx>)),
    BoolLiteral((bool, IntValue<'ctx>)),
}
