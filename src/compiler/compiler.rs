use inkwell::{
    AddressSpace,
    context::Context,
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};
use std::path::PathBuf;

use crate::{
    compiler::{
        blocks::{
            control::parse_control_stmt,
            data::parse_data_stmt,
            literal::parse_literal_expr,
            looks::{parse_looks_expr, parse_looks_stmt},
            motion::parse_motion_stmt,
            operator::parse_operator_expr,
            sensing::{parse_sensing_expr, parse_sensing_stmt},
        },
        types::{Builders, CompilerState},
    },
    jit,
    parser::types::{Expr, Keys::B, Stmt, Thread},
    types::ScratchProject,
};

pub struct CompilerOption {
    pub emit_llvm: Option<PathBuf>,
    pub run_jit: bool,
}

pub fn compiler(project: &ScratchProject, threads: &Vec<Thread>, option: CompilerOption) {
    let context = Context::create();
    let mut builders = Builders::new(&context, project);
    let mut thread_functions = Vec::with_capacity(threads.len());

    threads.iter().for_each(|v| {
        thread_functions.push(generate_thread_ir(&mut builders, v));
    });

    if let Some(emit_llvm) = option.emit_llvm {
        builders.module.print_to_file(emit_llvm).unwrap();
    }

    builders.module.verify().unwrap();

    if option.run_jit {
        jit::run(&builders, &thread_functions);
    }
}

pub fn generate_thread_ir(builders: &mut Builders, thread: &Thread) -> String {
    let ptr_type = builders.context.ptr_type(AddressSpace::default());
    let fn_type = builders.context.void_type().fn_type(&[ptr_type.into()], false);
    let function_name = builders.create_function_name();
    let function = builders.module.add_function(&function_name, fn_type, None);
    let entry = builders.context.append_basic_block(function, "entry");
    builders.builder.position_at_end(entry);
    let mut compiler_state: CompilerState = CompilerState {
        request_redraw: false,
        has_terminator: false,
    };
    for block in &thread.stmts {
        match block {
            Stmt::Motion(v) => parse_motion_stmt(builders, v, &function, thread.target_idx, &mut compiler_state),
            Stmt::Looks(v) => parse_looks_stmt(builders, v, &function, thread.target_idx, &mut compiler_state),
            Stmt::DataStmt(v) => parse_data_stmt(builders, v, &function, thread.target_idx, &mut compiler_state),
            Stmt::Control(v) => parse_control_stmt(builders, v, &function, thread.target_idx, &mut compiler_state),
            Stmt::Sensing(v) => parse_sensing_stmt(builders, v, &function, thread.target_idx, &mut compiler_state),
            _ => todo!("やります"),
        }
        if compiler_state.has_terminator {
            break;
        }
    }
    if compiler_state.request_redraw {
        builders
            .builder
            .build_call(
                builders.functions.wait_tick,
                &[builders.context.f64_type().const_float(builders.fps).into()],
                "forever_wait_tick",
            )
            .unwrap();
    }
    if !compiler_state.has_terminator {
        builders.builder.build_return(None).unwrap();
    }
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
        Expr::Looks(l) => parse_looks_expr(builders, l, function, target_idx),
        Expr::Sensing(l) => parse_sensing_expr(builders, l, function, target_idx),
        _ => todo!("やる"),
    }
}

pub enum ScratchReturnTypes<'ctx> {
    Number(FloatValue<'ctx>),
    String(PointerValue<'ctx>),
    Bool(IntValue<'ctx>),
    NumberLiteral((f64, FloatValue<'ctx>)),
    StringLiteral((String, PointerValue<'ctx>)),
    BoolLiteral((bool, IntValue<'ctx>)),
    Dynamic(PointerValue<'ctx>),
}
