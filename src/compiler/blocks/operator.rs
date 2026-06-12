use core::f64;

use inkwell::{
    FloatPredicate,
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};

use crate::{
    compiler::{
        compiler::{ScratchReturnTypes, generate_expr_ir},
        types::{Builders, DynamicKind, create_dynamic_struct_type},
        utils::{is_num, scratch_return_to_bool, scratch_return_to_number, scratch_return_to_string},
    },
    parser::types::{CalcOp, OperatorExpr},
};

fn build_rem<'ctx>(builders: &Builders<'ctx>, left: FloatValue<'ctx>, right: FloatValue<'ctx>) -> FloatValue<'ctx> {
    let rem = builders.builder.build_float_rem(left, right, "aaaa").unwrap();
    let rem = builders
        .builder
        .build_float_add(
            rem,
            builders
                .builder
                .build_select(
                    builders
                        .builder
                        .build_float_compare(
                            FloatPredicate::OLT,
                            rem,
                            builders.context.f64_type().const_float(0.0),
                            "aaaa",
                        )
                        .unwrap(),
                    right,
                    builders.context.f64_type().const_float(0.0),
                    "name",
                )
                .unwrap()
                .into_float_value(),
            "rem",
        )
        .unwrap();
    return rem;
}

fn build_float_is_nan<'ctx>(builders: &Builders<'ctx>, value: FloatValue<'ctx>) -> IntValue<'ctx> {
    builders
        .builder
        .build_float_compare(FloatPredicate::UNO, value, value, "is_nan")
        .unwrap()
}

fn build_string_compare_is_nan<'ctx>(builders: &Builders<'ctx>, value: PointerValue<'ctx>) -> IntValue<'ctx> {
    builders
        .builder
        .build_call(
            builders.functions.str_compare_is_nan,
            &[value.into()],
            "str_compare_is_nan",
        )
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_int_value()
}

fn compare_value_is_nan<'ctx>(
    builders: &Builders<'ctx>,
    value: &ScratchReturnTypes<'ctx>,
    function: &FunctionValue<'ctx>,
) -> IntValue<'ctx> {
    match value {
        ScratchReturnTypes::Number(v) => build_float_is_nan(builders, *v),
        ScratchReturnTypes::Bool(_) | ScratchReturnTypes::BoolLiteral(_) => {
            builders.context.bool_type().const_int(0, false)
        }
        ScratchReturnTypes::String(v) => build_string_compare_is_nan(builders, *v),
        ScratchReturnTypes::NumberLiteral((v, _)) => builders.context.bool_type().const_int(v.is_nan() as u64, false),
        ScratchReturnTypes::StringLiteral(v) => build_float_is_nan(
            builders,
            builders
                .context
                .f64_type()
                .const_float(v.0.parse::<f64>().unwrap_or(f64::NAN)),
        ),
        ScratchReturnTypes::Dynamic(v) => {
            let dynamic_struct = create_dynamic_struct_type(builders.context);
            let dispatch_block = builders
                .builder
                .get_insert_block()
                .expect("builder has no insert block");
            let kind_ptr = builders
                .builder
                .build_struct_gep(dynamic_struct, *v, 0, "dynamic_kind_ptr")
                .unwrap();
            let payload_ptr = builders
                .builder
                .build_struct_gep(dynamic_struct, *v, 1, "dynamic_payload_ptr")
                .unwrap();
            let kind = builders
                .builder
                .build_load(builders.context.i8_type(), kind_ptr, "dynamic_kind")
                .unwrap()
                .into_int_value();
            let payload = builders
                .builder
                .build_load(
                    builders.context.ptr_type(inkwell::AddressSpace::default()),
                    payload_ptr,
                    "dynamic_payload",
                )
                .unwrap()
                .into_pointer_value();
            let number_block = builders.context.append_basic_block(*function, "compare_number_nan");
            let string_block = builders.context.append_basic_block(*function, "compare_string_nan");
            let bool_block = builders.context.append_basic_block(*function, "compare_bool_nan");
            let finally = builders.context.append_basic_block(*function, "compare_nan_finally");
            builders
                .builder
                .build_switch(
                    kind,
                    finally,
                    &[
                        (
                            builders.context.i8_type().const_int(DynamicKind::Number as u64, false),
                            number_block,
                        ),
                        (
                            builders.context.i8_type().const_int(DynamicKind::String as u64, false),
                            string_block,
                        ),
                        (
                            builders.context.i8_type().const_int(DynamicKind::Bool as u64, false),
                            bool_block,
                        ),
                    ],
                )
                .unwrap();

            builders.builder.position_at_end(number_block);
            let number_value = builders
                .builder
                .build_load(builders.context.f64_type(), payload, "dynamic_number")
                .unwrap()
                .into_float_value();
            let number_ret = build_float_is_nan(builders, number_value);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(string_block);
            let string_ret = build_string_compare_is_nan(builders, payload);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(bool_block);
            let false_value = builders.context.bool_type().const_int(0, false);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(finally);
            let phi = builders
                .builder
                .build_phi(builders.context.bool_type(), "compare_is_nan")
                .unwrap();
            phi.add_incoming(&[
                (&false_value, dispatch_block),
                (&number_ret, number_block),
                (&string_ret, string_block),
                (&false_value, bool_block),
            ]);
            phi.as_basic_value().into_int_value()
        }
    }
}

fn build_compare<'ctx>(
    builders: &Builders<'ctx>,
    function: &FunctionValue<'ctx>,
    parsed_left: &ScratchReturnTypes<'ctx>,
    parsed_right: &ScratchReturnTypes<'ctx>,
    str_cmp_function: FunctionValue<'ctx>,
    str_cmp_name: &str,
    float_predicate: FloatPredicate,
    float_cmp_name: &str,
) -> ScratchReturnTypes<'ctx> {
    let left_compare_nan = compare_value_is_nan(builders, parsed_left, function);
    let right_compare_nan = compare_value_is_nan(builders, parsed_right, function);
    let is_string_compare = builders
        .builder
        .build_or(left_compare_nan, right_compare_nan, "is_string_compare")
        .unwrap();
    let string_compare_block = builders.context.append_basic_block(*function, "string_cmp");
    let number_compare_block = builders.context.append_basic_block(*function, "number_cmp");
    let finally_block = builders.context.append_basic_block(*function, "finally");
    builders
        .builder
        .build_conditional_branch(is_string_compare, string_compare_block, number_compare_block)
        .unwrap();

    builders.builder.position_at_end(string_compare_block);
    let left_hand = scratch_return_to_string(builders, parsed_left, function);
    let right_hand = scratch_return_to_string(builders, parsed_right, function);
    let string_cmp = builders
        .builder
        .build_call(str_cmp_function, &[left_hand.into(), right_hand.into()], str_cmp_name)
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_int_value();
    let string_compare_block = builders
        .builder
        .get_insert_block()
        .expect("builder has no insert block");
    builders.builder.build_unconditional_branch(finally_block).unwrap();

    builders.builder.position_at_end(number_compare_block);
    let left_hand = scratch_return_to_number(builders, parsed_left, function);
    let right_hand = scratch_return_to_number(builders, parsed_right, function);
    let number_cmp = builders
        .builder
        .build_float_compare(float_predicate, left_hand, right_hand, float_cmp_name)
        .unwrap();
    let number_compare_block = builders
        .builder
        .get_insert_block()
        .expect("builder has no insert block");
    builders.builder.build_unconditional_branch(finally_block).unwrap();

    builders.builder.position_at_end(finally_block);
    let phi = builders.builder.build_phi(builders.context.bool_type(), "phi").unwrap();
    phi.add_incoming(&[(&string_cmp, string_compare_block), (&number_cmp, number_compare_block)]);
    ScratchReturnTypes::Bool(phi.as_basic_value().into_int_value())
}

pub fn parse_operator_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &OperatorExpr,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        OperatorExpr::Add { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, target_idx),
                function,
            );
            ScratchReturnTypes::Number(
                builders
                    .builder
                    .build_float_add(parsed_left, parsed_right, "add")
                    .unwrap(),
            )
        }
        OperatorExpr::Sub { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, target_idx),
                function,
            );
            ScratchReturnTypes::Number(
                builders
                    .builder
                    .build_float_sub(parsed_left, parsed_right, "sub")
                    .unwrap(),
            )
        }
        OperatorExpr::Mul { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, target_idx),
                function,
            );
            ScratchReturnTypes::Number(
                builders
                    .builder
                    .build_float_mul(parsed_left, parsed_right, "mul")
                    .unwrap(),
            )
        }
        OperatorExpr::Div { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, target_idx),
                function,
            );
            ScratchReturnTypes::Number(
                builders
                    .builder
                    .build_float_div(parsed_left, parsed_right, "div")
                    .unwrap(),
            )
        }
        OperatorExpr::Random { from, to } => {
            let parsed_from = generate_expr_ir(builders, from, function, target_idx);
            let parsed_to = generate_expr_ir(builders, to, function, target_idx);
            let parsed_from_number = scratch_return_to_number(builders, &parsed_from, function);
            let parsed_to_number = scratch_return_to_number(builders, &parsed_to, function);
            let from_is_num = is_num(builders, parsed_from, function);
            let to_is_num = is_num(builders, parsed_to, function);
            let is_number_random = builders
                .builder
                .build_and(from_is_num, to_is_num, "is_number_random")
                .unwrap();
            let rand = builders
                .builder
                .build_call(builders.functions.rand, &[], "rand")
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_int_value();
            let rand_bits = builders
                .builder
                .build_right_shift(rand, builders.context.i64_type().const_int(12, false), false, "left")
                .unwrap();
            let rand_bits = builders
                .builder
                .build_or(
                    rand_bits,
                    builders.context.i64_type().const_int(0x3FF0000000000000, false),
                    "rand",
                )
                .unwrap();
            let rand_float = builders
                .builder
                .build_float_sub(
                    builders
                        .builder
                        .build_bit_cast(rand_bits, builders.context.f64_type(), "u64_to_f64")
                        .unwrap()
                        .into_float_value(),
                    builders.context.f64_type().const_float(1.0),
                    "rand",
                )
                .unwrap();
            let min = builders
                .builder
                .build_select(
                    builders
                        .builder
                        .build_float_compare(FloatPredicate::OLT, parsed_from_number, parsed_to_number, "a")
                        .unwrap(),
                    parsed_from_number,
                    parsed_to_number,
                    "from",
                )
                .unwrap()
                .into_float_value();
            let max = builders
                .builder
                .build_select(
                    builders
                        .builder
                        .build_float_compare(FloatPredicate::OLT, parsed_from_number, parsed_to_number, "a")
                        .unwrap(),
                    parsed_to_number,
                    parsed_from_number,
                    "to",
                )
                .unwrap()
                .into_float_value();
            let int_rand = {
                let range = builders
                    .builder
                    .build_float_add(
                        builders.builder.build_float_sub(max, min, "range_sub").unwrap(),
                        builders.context.f64_type().const_float(1.0),
                        "range_add",
                    )
                    .unwrap();

                let scaled = builders.builder.build_float_mul(rand_float, range, "scaled").unwrap();

                let floor = builders
                    .builder
                    .build_call(builders.functions.llvm_floor, &[scaled.into()], "floor_res")
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_float_value();

                builders.builder.build_float_add(floor, min, "int_rand").unwrap()
            };

            ScratchReturnTypes::Number(
                builders
                    .builder
                    .build_select(
                        is_number_random,
                        int_rand,
                        builders
                            .builder
                            .build_float_add(
                                min,
                                builders
                                    .builder
                                    .build_float_mul(
                                        rand_float,
                                        builders.builder.build_float_sub(max, min, "real_range").unwrap(),
                                        "real_scaled",
                                    )
                                    .unwrap(),
                                "real_rand",
                            )
                            .unwrap(),
                        "final_rand",
                    )
                    .unwrap()
                    .into_float_value(),
            )
        }
        OperatorExpr::GreaterThan { left, right } => {
            let parsed_left = generate_expr_ir(builders, left, function, target_idx);
            let parsed_right = generate_expr_ir(builders, right, function, target_idx);
            build_compare(
                builders,
                function,
                &parsed_left,
                &parsed_right,
                builders.functions.str_cmp_gt,
                "str_cmp_gt",
                FloatPredicate::OGT,
                "gt",
            )
        }
        OperatorExpr::LessThan { left, right } => {
            let parsed_left = generate_expr_ir(builders, left, function, target_idx);
            let parsed_right = generate_expr_ir(builders, right, function, target_idx);
            build_compare(
                builders,
                function,
                &parsed_left,
                &parsed_right,
                builders.functions.str_cmp_lt,
                "str_cmp_lt",
                FloatPredicate::OLT,
                "lt",
            )
        }
        OperatorExpr::Eq { left, right } => {
            let parsed_left = generate_expr_ir(builders, left, function, target_idx);
            let parsed_right = generate_expr_ir(builders, right, function, target_idx);
            build_compare(
                builders,
                function,
                &parsed_left,
                &parsed_right,
                builders.functions.str_cmp_eq,
                "str_cmp_eq",
                FloatPredicate::OEQ,
                "eq",
            )
        }
        OperatorExpr::And { left, right } => {
            if let Some(left_expr) = left
                && let Some(right_expr) = right
            {
                let left_parsed = generate_expr_ir(builders, left_expr, function, target_idx);
                let right_parsed = generate_expr_ir(builders, right_expr, function, target_idx);
                let left_bool = scratch_return_to_bool(builders, &left_parsed, function);
                let right_bool = scratch_return_to_bool(builders, &right_parsed, function);
                ScratchReturnTypes::Bool(builders.builder.build_and(left_bool, right_bool, "and").unwrap())
            } else {
                ScratchReturnTypes::BoolLiteral((false, builders.context.bool_type().const_int(0, false)))
            }
        }
        OperatorExpr::Or { left, right } => {
            if left.is_some() || right.is_some() {
                let left_parsed = if let Some(left_some) = left {
                    scratch_return_to_bool(
                        builders,
                        &generate_expr_ir(builders, left_some, function, target_idx),
                        function,
                    )
                } else {
                    builders.context.bool_type().const_int(0, false)
                };
                let right_parsed = if let Some(right_some) = right {
                    scratch_return_to_bool(
                        builders,
                        &generate_expr_ir(builders, right_some, function, target_idx),
                        function,
                    )
                } else {
                    builders.context.bool_type().const_int(0, false)
                };
                ScratchReturnTypes::Bool(builders.builder.build_or(left_parsed, right_parsed, "or").unwrap())
            } else {
                ScratchReturnTypes::BoolLiteral((false, builders.context.bool_type().const_int(0, false)))
            }
        }
        OperatorExpr::Not { target } => {
            if let Some(target_some) = target {
                let target_parsed = scratch_return_to_bool(
                    builders,
                    &generate_expr_ir(builders, target_some, function, target_idx),
                    function,
                );
                ScratchReturnTypes::Bool(builders.builder.build_not(target_parsed, "not").unwrap())
            } else {
                ScratchReturnTypes::BoolLiteral((true, builders.context.bool_type().const_int(1, false)))
            }
        }
        OperatorExpr::Mod { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, target_idx),
                function,
            );
            ScratchReturnTypes::Number(build_rem(builders, parsed_left, parsed_right))
        }
        OperatorExpr::Round { target } => {
            let parsed_target = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, target, function, target_idx),
                function,
            );
            ScratchReturnTypes::Number(
                builders
                    .builder
                    .build_select(
                        builders
                            .builder
                            .build_or(
                                builders
                                    .builder
                                    .build_float_compare(
                                        FloatPredicate::OEQ,
                                        parsed_target,
                                        builders.context.f64_type().const_float(-0.0),
                                        "is_minus_zero",
                                    )
                                    .unwrap(),
                                builders
                                    .builder
                                    .build_and(
                                        builders
                                            .builder
                                            .build_float_compare(
                                                FloatPredicate::OGE,
                                                parsed_target,
                                                builders.context.f64_type().const_float(-0.5),
                                                "ge_half",
                                            )
                                            .unwrap(),
                                        builders
                                            .builder
                                            .build_float_compare(
                                                FloatPredicate::OLT,
                                                parsed_target,
                                                builders.context.f64_type().const_float(0.0),
                                                "lt_zero",
                                            )
                                            .unwrap(),
                                        "gt_half",
                                    )
                                    .unwrap(),
                                "or_or_or",
                            )
                            .unwrap(),
                        builders.context.f64_type().const_float(-0.0),
                        builders
                            .builder
                            .build_call(
                                builders.functions.llvm_floor,
                                &[builders
                                    .builder
                                    .build_float_add(
                                        parsed_target,
                                        builders.context.f64_type().const_float(0.5),
                                        "plus_half",
                                    )
                                    .unwrap()
                                    .into()],
                                "round_select",
                            )
                            .unwrap()
                            .try_as_basic_value()
                            .basic()
                            .unwrap()
                            .into_float_value(),
                        "round",
                    )
                    .unwrap()
                    .into_float_value(),
            )
        }
        OperatorExpr::Calc { target, op } => {
            let parsed_target = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, target, function, target_idx),
                function,
            );
            let parsed_target_old = &parsed_target;
            let parsed_target = if matches!(op, CalcOp::Sin) || matches!(op, CalcOp::Cos) || matches!(op, CalcOp::Tan) {
                builders
                    .builder
                    .build_float_mul(
                        parsed_target,
                        builders.context.f64_type().const_float(f64::consts::PI / 180.0),
                        "degrees_to_rad",
                    )
                    .unwrap()
            } else {
                parsed_target
            };
            let ret_val = builders
                .builder
                .build_call(
                    match op {
                        CalcOp::Abs => builders.functions.llvm_abs,
                        CalcOp::Floor => builders.functions.llvm_floor,
                        CalcOp::Ceil => builders.functions.llvm_ceil,
                        CalcOp::Sqrt => builders.functions.llvm_sqrt,
                        CalcOp::Sin => builders.functions.llvm_sin,
                        CalcOp::Cos => builders.functions.llvm_cos,
                        CalcOp::Tan => builders.functions.llvm_tan,
                        CalcOp::Asin => builders.functions.llvm_asin,
                        CalcOp::Acos => builders.functions.llvm_acos,
                        CalcOp::Atan => builders.functions.llvm_atan,
                        CalcOp::LogE => builders.functions.llvm_loge,
                        CalcOp::Log10 => builders.functions.llvm_log10,
                        CalcOp::PowE => builders.functions.llvm_exp,
                        CalcOp::Pow10 => builders.functions.llvm_pow10,
                    },
                    &[parsed_target.into()],
                    "float_calc",
                )
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value();
            let ret_val = if matches!(op, CalcOp::Sin) || matches!(op, CalcOp::Cos) || matches!(op, CalcOp::Tan) {
                let tm = if matches!(op, CalcOp::Tan) {
                    builders
                        .builder
                        .build_select(
                            builders
                                .builder
                                .build_float_compare(
                                    FloatPredicate::OEQ,
                                    build_rem(
                                        builders,
                                        *parsed_target_old,
                                        builders.context.f64_type().const_float(360.0),
                                    ),
                                    builders.context.f64_type().const_float(90.0),
                                    "tan_inf",
                                )
                                .unwrap(),
                            builders.context.f64_type().const_float(f64::INFINITY),
                            builders
                                .builder
                                .build_select(
                                    builders
                                        .builder
                                        .build_float_compare(
                                            FloatPredicate::OEQ,
                                            build_rem(
                                                builders,
                                                *parsed_target_old,
                                                builders.context.f64_type().const_float(360.0),
                                            ),
                                            builders.context.f64_type().const_float(270.0),
                                            "tan_inf",
                                        )
                                        .unwrap(),
                                    builders.context.f64_type().const_float(-f64::INFINITY),
                                    ret_val,
                                    "inf",
                                )
                                .unwrap()
                                .into_float_value(),
                            "inf",
                        )
                        .unwrap()
                        .into_float_value()
                } else {
                    ret_val
                };
                builders
                    .builder
                    .build_float_mul(
                        builders
                            .builder
                            .build_call(
                                builders.functions.llvm_floor,
                                &[builders
                                    .builder
                                    .build_float_add(
                                        builders
                                            .builder
                                            .build_float_mul(
                                                tm,
                                                builders.context.f64_type().const_float(10.0_f64.powi(10)),
                                                "mul_10_000_000_000",
                                            )
                                            .unwrap(),
                                        builders.context.f64_type().const_float(0.5),
                                        "round_hosei",
                                    )
                                    .unwrap()
                                    .into()],
                                "round_round",
                            )
                            .unwrap()
                            .try_as_basic_value()
                            .basic()
                            .unwrap()
                            .into_float_value(),
                        builders.context.f64_type().const_float(10.0_f64.powi(-10)),
                        "aaaa",
                    )
                    .unwrap()
            } else if matches!(op, CalcOp::Asin) || matches!(op, CalcOp::Acos) || matches!(op, CalcOp::Atan) {
                builders
                    .builder
                    .build_float_mul(
                        ret_val,
                        builders.context.f64_type().const_float(180.0 / f64::consts::PI),
                        "rad_to_degrees",
                    )
                    .unwrap()
            } else {
                ret_val
            };
            ScratchReturnTypes::Number(ret_val)
        }
        _ => todo!("あとでやる"),
    }
}

#[cfg(test)]
mod tests {
    use inkwell::{AddressSpace, context::Context, values::AnyValue};
    use serde_json::json;

    use super::*;
    use crate::{
        parser::types::{Expr, Literal},
        types::ScratchProject,
    };

    fn empty_project() -> ScratchProject {
        serde_json::from_value(json!({
            "meta": {
                "semver": "3.0.0",
                "vm": null,
                "agent": null,
                "origin": null
            },
            "targets": [{
                "isStage": true,
                "name": "Stage",
                "currentCostume": 0,
                "blocks": {},
                "variables": {},
                "lists": {},
                "broadcasts": {},
                "comments": null,
                "costumes": [],
                "sounds": [],
                "tempo": null,
                "videoTransparency": null,
                "videoState": null,
                "layerOrder": null,
                "volume": null
            }]
        }))
        .unwrap()
    }

    fn project_with_variable() -> ScratchProject {
        serde_json::from_value(json!({
            "meta": {
                "semver": "3.0.0",
                "vm": null,
                "agent": null,
                "origin": null
            },
            "targets": [{
                "isStage": true,
                "name": "Stage",
                "currentCostume": 0,
                "blocks": {},
                "variables": {
                    "score": ["score", "50"]
                },
                "lists": {},
                "broadcasts": {},
                "comments": null,
                "costumes": [],
                "sounds": [],
                "tempo": null,
                "videoTransparency": null,
                "videoState": null,
                "layerOrder": null,
                "volume": null
            }]
        }))
        .unwrap()
    }

    fn test_function<'ctx>(context: &'ctx Context, builders: &Builders<'ctx>) -> FunctionValue<'ctx> {
        let fn_type = context
            .void_type()
            .fn_type(&[context.ptr_type(AddressSpace::default()).into()], false);
        let function = builders.module.add_function("test_func", fn_type, None);
        let entry = context.append_basic_block(function, "entry");
        builders.builder.position_at_end(entry);
        function
    }

    fn literal(value: &str) -> Box<Expr> {
        Box::new(Expr::Literal(Literal::String(value.to_string())))
    }

    fn verify_dynamic_compare(expr: OperatorExpr) {
        let context = Context::create();
        let project = project_with_variable();
        let builders = Builders::new(&context, &project);
        let function = test_function(&context, &builders);

        let _ = parse_operator_expr(&builders, &expr, &function, 0);
        builders.builder.build_return(None).unwrap();

        assert!(builders.module.verify().is_ok(), "{}", builders.module.to_string());
    }

    fn variable_score() -> Box<Expr> {
        Box::new(Expr::Literal(Literal::Variable {
            target: "score".to_string(),
        }))
    }

    fn number_expr(value: &str) -> Box<Expr> {
        Box::new(Expr::Literal(Literal::Number(value.to_string())))
    }

    #[test]
    fn operator_or_uses_boolean_or_for_literal_truthiness() {
        let context = Context::create();
        let project = empty_project();
        let builders = Builders::new(&context, &project);
        let function = test_function(&context, &builders);
        let expr = OperatorExpr::Or {
            left: Some(literal("true")),
            right: Some(literal("false")),
        };

        let result = parse_operator_expr(&builders, &expr, &function, 0);

        match result {
            ScratchReturnTypes::Bool(value) => {
                assert_eq!(value.print_to_string().to_string(), "i1 true");
            }
            _ => panic!("expected boolean result"),
        }
    }

    #[test]
    fn operator_or_with_no_inputs_matches_scratch_false_default() {
        let context = Context::create();
        let project = empty_project();
        let builders = Builders::new(&context, &project);
        let function = test_function(&context, &builders);
        let expr = OperatorExpr::Or {
            left: None,
            right: None,
        };

        let result = parse_operator_expr(&builders, &expr, &function, 0);

        match result {
            ScratchReturnTypes::BoolLiteral((value, llvm_value)) => {
                assert!(!value);
                assert_eq!(llvm_value.print_to_string().to_string(), "i1 false");
            }
            _ => panic!("expected boolean literal result"),
        }
    }

    #[test]
    fn dynamic_less_than_builds_verifiable_phi_incoming_blocks() {
        verify_dynamic_compare(OperatorExpr::LessThan {
            left: variable_score(),
            right: number_expr("100"),
        });
    }

    #[test]
    fn dynamic_greater_than_builds_verifiable_phi_incoming_blocks() {
        verify_dynamic_compare(OperatorExpr::GreaterThan {
            left: variable_score(),
            right: number_expr("100"),
        });
    }

    #[test]
    fn dynamic_equals_builds_verifiable_phi_incoming_blocks() {
        verify_dynamic_compare(OperatorExpr::Eq {
            left: variable_score(),
            right: number_expr("100"),
        });
    }
}
