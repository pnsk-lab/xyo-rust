use inkwell::{
    AddressSpace, OptimizationLevel,
    context::Context,
    passes::PassBuilderOptions,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    types::StructType,
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};

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

pub fn compiler(project: &ScratchProject, threads: &Vec<Thread>) {
    Target::initialize_all(&InitializationConfig::default());
    let context = Context::create();
    let mut builders = Builders::new(&context, project);
    let mut strings: Vec<String> = vec![];

    threads.iter().for_each(|v| {
        generate_thread_ir(&mut builders, v, &mut strings);
    });

    let target_triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();
    println!("cpu: {cpu}, features: {features}");
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &cpu,
            &features,
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
    println!("{}", builders.module.to_string())
}

pub fn generate_thread_ir(builders: &mut Builders, thread: &Thread, strings: &mut Vec<String>) {
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
            Stmt::Motion(v) => {
                parse_motion_stmt(builders, v, &function, strings, thread.target_idx)
            }
            _ => todo!("やります"),
        }
    }
    builders.builder.build_return(None).unwrap();
}

pub fn generate_expr_ir<'ctx>(
    builders: &Builders<'ctx>,
    expr: &Expr,
    function: &FunctionValue<'ctx>,
    strings: &mut Vec<String>,
    target_idx: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        Expr::Literal(l) => parse_literal_expr(builders, l, function, strings, target_idx),
        Expr::Operator(l) => parse_operator_expr(builders, l, function, strings, target_idx),
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
