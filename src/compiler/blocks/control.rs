use inkwell::values::FunctionValue;

use crate::{
    compiler::{
        compiler::{generate_expr_ir, generate_thread_ir},
        types::{Builders, CompilerState},
        utils::scratch_return_to_number,
    },
    parser::types::{ControlStmt, Thread},
};

pub fn parse_control_stmt<'ctx>(
    builders: &mut Builders<'ctx>,
    stmt: &ControlStmt,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
    compiler_state: &mut CompilerState,
) {
    match stmt {
        ControlStmt::Forever { substack } => {
            if let Some(substack) = substack {
                let current_block = builders
                    .builder
                    .get_insert_block()
                    .expect("builder has no insert block");
                let thread: Thread = Thread {
                    hat: None,
                    stmts: substack.clone(),
                    target_idx,
                };
                let func_name = generate_thread_ir(builders, &thread);
                let func = builders.module.get_function(&func_name).unwrap();
                builders.builder.position_at_end(current_block);
                let loop_label = builders.context.append_basic_block(*function, "forever");
                builders.builder.build_unconditional_branch(loop_label).unwrap();
                builders.builder.position_at_end(loop_label);
                builders
                    .builder
                    .build_call(func, &[function.get_first_param().unwrap().into()], "forever_thread")
                    .unwrap();
                builders.builder.build_unconditional_branch(loop_label).unwrap();
                compiler_state.has_terminator = true;
            }
        }
        ControlStmt::Repeat { times, substack } => {
            if let Some(substack) = substack {
                let current_block = builders
                    .builder
                    .get_insert_block()
                    .expect("builder has no insert block");
                let thread: Thread = Thread {
                    hat: None,
                    stmts: substack.clone(),
                    target_idx,
                };
                let counter = builders
                    .builder
                    .build_alloca(builders.context.f64_type(), "counter_ptr")
                    .unwrap();
                builders
                    .builder
                    .build_store(counter, builders.context.f64_type().const_float(0.0))
                    .unwrap();
                let func_name = generate_thread_ir(builders, &thread);
                let func = builders.module.get_function(&func_name).unwrap();
                builders.builder.position_at_end(current_block);
                let loop_label = builders.context.append_basic_block(*function, "repeat");
                builders.builder.build_unconditional_branch(loop_label).unwrap();
                builders.builder.position_at_end(loop_label);
                let times_value = builders
                    .builder
                    .build_call(
                        builders.functions.llvm_ceil,
                        &[scratch_return_to_number(
                            builders,
                            &generate_expr_ir(builders, times, function, target_idx),
                            function,
                        )
                        .into()],
                        "round",
                    )
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_float_value();
                let inner = builders.context.append_basic_block(*function, "inner_content");
                let out = builders.context.append_basic_block(*function, "out");
                builders
                    .builder
                    .build_conditional_branch(
                        builders
                            .builder
                            .build_float_compare(
                                inkwell::FloatPredicate::OLT,
                                builders
                                    .builder
                                    .build_load(builders.context.f64_type(), counter, "counter")
                                    .unwrap()
                                    .into_float_value(),
                                times_value,
                                "times",
                            )
                            .unwrap(),
                        inner,
                        out,
                    )
                    .unwrap();
                builders.builder.position_at_end(inner);
                builders
                    .builder
                    .build_call(func, &[function.get_first_param().unwrap().into()], "repeat_thread")
                    .unwrap();
                let new_counter_value = builders
                    .builder
                    .build_float_add(
                        builders
                            .builder
                            .build_load(builders.context.f64_type(), counter, "counter_value")
                            .unwrap()
                            .into_float_value(),
                        builders.context.f64_type().const_float(1.0),
                        "buhi",
                    )
                    .unwrap();
                builders.builder.build_store(counter, new_counter_value).unwrap();
                builders.builder.build_unconditional_branch(loop_label).unwrap();
                builders.builder.position_at_end(out);
            }
        }
        _ => todo!("未実装"),
    }
}
