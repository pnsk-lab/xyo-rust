use inkwell::{
    AddressSpace,
    context::Context,
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};

use crate::{
    compiler::{
        blocks::{
            literal::parse_literal_expr, motion::parse_motion_stmt, operator::parse_operator_expr,
        },
        types::Builders,
    },
    jit,
    parser::types::{Expr, Stmt, Thread},
    types::ScratchProject,
};

pub fn compiler(project: &ScratchProject, threads: &Vec<Thread>) {
    let context = Context::create();
    let mut builders = Builders::new(&context, project);
    let mut thread_functions = Vec::with_capacity(threads.len());

    threads.iter().for_each(|v| {
        thread_functions.push(generate_thread_ir(&mut builders, v));
    });

    builders.module.print_to_file("path.ll").unwrap();

    jit::run(&builders, &thread_functions);
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
