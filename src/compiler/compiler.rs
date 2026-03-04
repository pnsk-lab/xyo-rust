use inkwell::{
    AddressSpace, OptimizationLevel,
    context::Context,
    passes::PassBuilderOptions,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    values::{FloatValue, FunctionValue, IntValue},
};

use crate::{
    compiler::{
        blocks::{literal::parse_literal_expr, motion::parse_motion_stmt},
        types::Builders,
    },
    parser::types::{Expr, Stmt, Thread},
    types::ScratchProject,
};

pub fn compiler(project: &ScratchProject, threads: &Vec<Thread>) {
    Target::initialize_all(&InitializationConfig::default());
    let context = Context::create();
    let mut builders = Builders::new(&context);
    let mut strings: Vec<String> = vec![];
    let project = project;

    threads.iter().for_each(|v| {
        generate_thread_ir(&mut builders, v, &mut strings);
    });

    // println!("IR before optimization:\n{}", builders.module.to_string());

    // let target_triple = TargetMachine::get_default_triple();
    // let target = Target::from_triple(&target_triple).unwrap();
    // let target_machine = target
    //     .create_target_machine(
    //         &target_triple,
    //         "",
    //         "",
    //         OptimizationLevel::None,
    //         RelocMode::PIC,
    //         CodeModel::Default,
    //     )
    //     .unwrap();

    // let passes = "default<O3>,loop-vectorize,loop-unroll";
    // let pass_builder_options = PassBuilderOptions::create();
    // builders
    //     .module
    //     .run_passes(passes, &target_machine, pass_builder_options)
    //     .unwrap();
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
            Stmt::Motion(v) => parse_motion_stmt(builders, v, &function, strings),
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
) -> FloatValue<'ctx> {
    let parse_result = match expr {
        Expr::Literal(l) => parse_literal_expr(builders, l, function, strings),
        _ => todo!("やる"),
    };
    scratch_return_to_number(builders, parse_result, function)
}

pub enum ScratchReturnTypes<'ctx> {
    Number(FloatValue<'ctx>),
    String(IntValue<'ctx>),
    Bool(IntValue<'ctx>),
}

fn scratch_return_to_number<'ctx>(
    builders: &Builders<'ctx>,
    from: ScratchReturnTypes<'ctx>,
    func: &FunctionValue<'ctx>,
) -> FloatValue<'ctx> {
    match from {
        ScratchReturnTypes::Number(v) => v,
        ScratchReturnTypes::Bool(v) => builders
            .builder
            .build_select(
                v,
                builders.context.f64_type().const_float(1.0),
                builders.context.f64_type().const_float(0.0),
                "num_bool",
            )
            .unwrap()
            .into_float_value(),
        ScratchReturnTypes::String(v) => {
            let p = func.get_first_param().unwrap().into_pointer_value();
            builders
                .builder
                .build_call(
                    builders.functions.str_to_num,
                    &[p.into(), v.into()],
                    "xyo_str_to_num",
                )
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value()
        }
    }
}
