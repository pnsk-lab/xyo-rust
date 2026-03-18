use core::f64;

use inkwell::{
    FloatPredicate,
    values::{FloatValue, FunctionValue},
};

use crate::{
    compiler::{
        compiler::{ScratchReturnTypes, generate_expr_ir},
        types::Builders,
        utils::{
            is_num, scratch_return_to_bool, scratch_return_to_number, scratch_return_to_string,
        },
    },
    parser::types::{CalcOp, OperatorExpr},
};

fn build_rem<'ctx>(
    builders: &Builders<'ctx>,
    left: FloatValue<'ctx>,
    right: FloatValue<'ctx>,
) -> FloatValue<'ctx> {
    let rem = builders
        .builder
        .build_float_rem(left, right, "aaaa")
        .unwrap();
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

pub fn parse_operator_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &OperatorExpr,
    function: &FunctionValue<'ctx>,
    strings: &mut Vec<String>,
    target_idx: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        OperatorExpr::Add { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, strings, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings, target_idx),
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
                &generate_expr_ir(builders, left, function, strings, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings, target_idx),
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
                &generate_expr_ir(builders, left, function, strings, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings, target_idx),
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
                &generate_expr_ir(builders, left, function, strings, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings, target_idx),
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
            let parsed_from = generate_expr_ir(builders, from, function, strings, target_idx);
            let parsed_to = generate_expr_ir(builders, to, function, strings, target_idx);
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
                .build_right_shift(
                    rand,
                    builders.context.i64_type().const_int(12, false),
                    false,
                    "left",
                )
                .unwrap();
            let rand_bits = builders
                .builder
                .build_or(
                    rand_bits,
                    builders
                        .context
                        .i64_type()
                        .const_int(0x3FF0000000000000, false),
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
                        .build_float_compare(
                            FloatPredicate::OLT,
                            parsed_from_number,
                            parsed_to_number,
                            "a",
                        )
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
                        .build_float_compare(
                            FloatPredicate::OLT,
                            parsed_from_number,
                            parsed_to_number,
                            "a",
                        )
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
                        builders
                            .builder
                            .build_float_sub(max, min, "range_sub")
                            .unwrap(),
                        builders.context.f64_type().const_float(1.0),
                        "range_add",
                    )
                    .unwrap();

                let scaled = builders
                    .builder
                    .build_float_mul(rand_float, range, "scaled")
                    .unwrap();

                let floor = builders
                    .builder
                    .build_call(builders.functions.llvm_floor, &[scaled.into()], "floor")
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_float_value();

                builders
                    .builder
                    .build_float_add(floor, min, "int_rand")
                    .unwrap()
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
                                        builders
                                            .builder
                                            .build_float_sub(max, min, "real_range")
                                            .unwrap(),
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
            let parsed_left = &generate_expr_ir(builders, left, function, strings, target_idx);
            let parsed_right = &generate_expr_ir(builders, right, function, strings, target_idx);
            let is_parsed_left_string = matches!(parsed_left, ScratchReturnTypes::String(_))
                || matches!(parsed_left, ScratchReturnTypes::StringLiteral(_));
            let is_parsed_right_string = matches!(parsed_right, ScratchReturnTypes::String(_))
                || matches!(parsed_right, ScratchReturnTypes::StringLiteral(_));
            let is_string_compare = is_parsed_left_string || is_parsed_right_string;
            if is_string_compare {
                let left_hand = scratch_return_to_string(builders, parsed_left, function, strings);
                let right_hand =
                    scratch_return_to_string(builders, parsed_right, function, strings);
                let p = function.get_first_param().unwrap().into_pointer_value();
                let cmp = builders
                    .builder
                    .build_call(
                        builders.functions.str_cmp_gt,
                        &[p.into(), left_hand.into(), right_hand.into()],
                        "str_cmp_gt",
                    )
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_int_value();
                ScratchReturnTypes::Bool(cmp)
            } else {
                let left_hand = scratch_return_to_number(builders, parsed_left, function);
                let right_hand = scratch_return_to_number(builders, parsed_right, function);
                ScratchReturnTypes::Bool(
                    builders
                        .builder
                        .build_float_compare(FloatPredicate::OGT, left_hand, right_hand, "gt")
                        .unwrap(),
                )
            }
        }
        OperatorExpr::LessThan { left, right } => {
            let parsed_left = &generate_expr_ir(builders, left, function, strings, target_idx);
            let parsed_right = &generate_expr_ir(builders, right, function, strings, target_idx);
            let is_parsed_left_string = matches!(parsed_left, ScratchReturnTypes::String(_))
                || matches!(parsed_left, ScratchReturnTypes::StringLiteral(_));
            let is_parsed_right_string = matches!(parsed_right, ScratchReturnTypes::String(_))
                || matches!(parsed_right, ScratchReturnTypes::StringLiteral(_));
            let is_string_compare = is_parsed_left_string || is_parsed_right_string;
            if is_string_compare {
                let left_hand = scratch_return_to_string(builders, parsed_left, function, strings);
                let right_hand =
                    scratch_return_to_string(builders, parsed_right, function, strings);
                let p = function.get_first_param().unwrap().into_pointer_value();
                let cmp = builders
                    .builder
                    .build_call(
                        builders.functions.str_cmp_lt,
                        &[p.into(), left_hand.into(), right_hand.into()],
                        "str_cmp_lt",
                    )
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_int_value();
                ScratchReturnTypes::Bool(cmp)
            } else {
                let left_hand = scratch_return_to_number(builders, parsed_left, function);
                let right_hand = scratch_return_to_number(builders, parsed_right, function);
                ScratchReturnTypes::Bool(
                    builders
                        .builder
                        .build_float_compare(FloatPredicate::OLT, left_hand, right_hand, "lt")
                        .unwrap(),
                )
            }
        }
        OperatorExpr::Eq { left, right } => {
            let parsed_left = &generate_expr_ir(builders, left, function, strings, target_idx);
            let parsed_right = &generate_expr_ir(builders, right, function, strings, target_idx);
            let is_parsed_left_string = matches!(parsed_left, ScratchReturnTypes::String(_))
                || matches!(parsed_left, ScratchReturnTypes::StringLiteral(_));
            let is_parsed_right_string = matches!(parsed_right, ScratchReturnTypes::String(_))
                || matches!(parsed_right, ScratchReturnTypes::StringLiteral(_));
            let is_string_compare = is_parsed_left_string || is_parsed_right_string;
            if is_string_compare {
                let left_hand = scratch_return_to_string(builders, parsed_left, function, strings);
                let right_hand =
                    scratch_return_to_string(builders, parsed_right, function, strings);
                let p = function.get_first_param().unwrap().into_pointer_value();
                let cmp = builders
                    .builder
                    .build_call(
                        builders.functions.str_cmp_eq,
                        &[p.into(), left_hand.into(), right_hand.into()],
                        "str_cmp_eq",
                    )
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_int_value();
                ScratchReturnTypes::Bool(cmp)
            } else {
                let left_hand = scratch_return_to_number(builders, parsed_left, function);
                let right_hand = scratch_return_to_number(builders, parsed_right, function);
                ScratchReturnTypes::Bool(
                    builders
                        .builder
                        .build_float_compare(FloatPredicate::OEQ, left_hand, right_hand, "eq")
                        .unwrap(),
                )
            }
        }
        OperatorExpr::And { left, right } => {
            if let Some(left_expr) = left
                && let Some(right_expr) = right
            {
                let left_parsed =
                    generate_expr_ir(builders, left_expr, function, strings, target_idx);
                let right_parsed =
                    generate_expr_ir(builders, right_expr, function, strings, target_idx);
                let left_bool = scratch_return_to_bool(builders, &left_parsed, function, strings);
                let right_bool = scratch_return_to_bool(builders, &right_parsed, function, strings);
                ScratchReturnTypes::Bool(
                    builders
                        .builder
                        .build_and(left_bool, right_bool, "and")
                        .unwrap(),
                )
            } else {
                ScratchReturnTypes::BoolLiteral((
                    false,
                    builders.context.bool_type().const_int(0, false),
                ))
            }
        }
        OperatorExpr::Or { left, right } => {
            if left.is_some() || right.is_some() {
                let left_parsed = if let Some(left_some) = left {
                    scratch_return_to_bool(
                        builders,
                        &generate_expr_ir(builders, left_some, function, strings, target_idx),
                        function,
                        strings,
                    )
                } else {
                    builders.context.bool_type().const_int(0, false)
                };
                let right_parsed = if let Some(right_some) = right {
                    scratch_return_to_bool(
                        builders,
                        &generate_expr_ir(builders, right_some, function, strings, target_idx),
                        function,
                        strings,
                    )
                } else {
                    builders.context.bool_type().const_int(0, false)
                };
                ScratchReturnTypes::Bool(
                    builders
                        .builder
                        .build_and(left_parsed, right_parsed, "or")
                        .unwrap(),
                )
            } else {
                ScratchReturnTypes::BoolLiteral((
                    false,
                    builders.context.bool_type().const_int(0, false),
                ))
            }
        }
        OperatorExpr::Not { target } => {
            if let Some(target_some) = target {
                let target_parsed = scratch_return_to_bool(
                    builders,
                    &generate_expr_ir(builders, target_some, function, strings, target_idx),
                    function,
                    strings,
                );
                ScratchReturnTypes::Bool(builders.builder.build_not(target_parsed, "not").unwrap())
            } else {
                ScratchReturnTypes::BoolLiteral((
                    true,
                    builders.context.bool_type().const_int(1, false),
                ))
            }
        }
        OperatorExpr::Mod { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, strings, target_idx),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings, target_idx),
                function,
            );
            ScratchReturnTypes::Number(build_rem(builders, parsed_left, parsed_right))
        }
        OperatorExpr::Round { target } => {
            let parsed_target = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, target, function, strings, target_idx),
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
                &generate_expr_ir(builders, target, function, strings, target_idx),
                function,
            );
            let parsed_target_old = &parsed_target;
            let parsed_target = if matches!(op, CalcOp::Sin)
                || matches!(op, CalcOp::Cos)
                || matches!(op, CalcOp::Tan)
            {
                builders
                    .builder
                    .build_float_mul(
                        parsed_target,
                        builders
                            .context
                            .f64_type()
                            .const_float(f64::consts::PI / 180.0),
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
            let ret_val = if matches!(op, CalcOp::Sin)
                || matches!(op, CalcOp::Cos)
                || matches!(op, CalcOp::Tan)
            {
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
                                                builders
                                                    .context
                                                    .f64_type()
                                                    .const_float(10.0_f64.powi(10)),
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
            } else if matches!(op, CalcOp::Asin)
                || matches!(op, CalcOp::Acos)
                || matches!(op, CalcOp::Atan)
            {
                builders
                    .builder
                    .build_float_mul(
                        ret_val,
                        builders
                            .context
                            .f64_type()
                            .const_float(180.0 / f64::consts::PI),
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
