use inkwell::values::FunctionValue;

use crate::{
    compiler::{
        compiler::ScratchReturnTypes,
        types::{Builders, CompilerState},
    },
    parser::types::{SensingExpr, SensingStmt},
};

pub fn parse_sensing_stmt<'ctx>(
    builders: &mut Builders<'ctx>,
    stmt: &SensingStmt,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
    _compiler_state: &mut CompilerState,
) {
    match stmt {
        SensingStmt::ResetTimer => {
            builders
                .builder
                .build_store(
                    builders.timer.as_pointer_value(),
                    builders
                        .builder
                        .build_call(builders.functions.get_now, &[], "xyo_now_ns")
                        .unwrap()
                        .try_as_basic_value()
                        .basic()
                        .unwrap()
                        .into_int_value(),
                )
                .unwrap();
        }
        _ => todo!("未実装biyolololon"),
    }
}

pub fn parse_sensing_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &SensingExpr,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        SensingExpr::Timer => ScratchReturnTypes::Number(
            builders
                .builder
                .build_float_div(
                    builders
                        .builder
                        .build_signed_int_to_float(
                            builders
                                .builder
                                .build_int_signed_div(
                                    builders
                                        .builder
                                        .build_int_sub(
                                            builders
                                                .builder
                                                .build_call(builders.functions.get_now, &[], "now")
                                                .unwrap()
                                                .try_as_basic_value()
                                                .basic()
                                                .unwrap()
                                                .into_int_value(),
                                            builders
                                                .builder
                                                .build_load(
                                                    builders.context.i64_type(),
                                                    builders.timer.as_pointer_value(),
                                                    "timer",
                                                )
                                                .unwrap()
                                                .into_int_value(),
                                            "name",
                                        )
                                        .unwrap(),
                                    builders.context.i64_type().const_int(1_000_000, false),
                                    "name",
                                )
                                .unwrap(),
                            builders.context.f64_type(),
                            "i64_to_f64",
                        )
                        .unwrap(),
                    builders.context.f64_type().const_float(1_000.0),
                    "phantom",
                )
                .unwrap(),
        ),
        _ => todo!("未実装びよーん"),
    }
}
