use std::string;

use inkwell::{FloatPredicate, values::FunctionValue};

use crate::{
    compiler::{
        compiler::{ScratchReturnTypes, generate_expr_ir},
        types::Builders,
        utils::{is_num, scratch_return_to_number, scratch_return_to_string},
    },
    parser::types::OperatorExpr,
};

pub fn parse_operator_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &OperatorExpr,
    function: &FunctionValue<'ctx>,
    strings: &mut Vec<String>,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        OperatorExpr::Add { left, right } => {
            let parsed_left = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, left, function, strings),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings),
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
                &generate_expr_ir(builders, left, function, strings),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings),
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
                &generate_expr_ir(builders, left, function, strings),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings),
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
                &generate_expr_ir(builders, left, function, strings),
                function,
            );
            let parsed_right = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, right, function, strings),
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
            let parsed_from = generate_expr_ir(builders, from, function, strings);
            let parsed_to = generate_expr_ir(builders, to, function, strings);
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
            let parsed_left = &generate_expr_ir(builders, left, function, strings);
            let parsed_right = &generate_expr_ir(builders, right, function, strings);
            let is_parsed_left_string = matches!(parsed_right, ScratchReturnTypes::String(_))
                || matches!(parsed_left, ScratchReturnTypes::StringLiteral(_));
            let is_parsed_right_string = matches!(parsed_right, ScratchReturnTypes::String(_))
                || matches!(parsed_left, ScratchReturnTypes::StringLiteral(_));
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
                let right_hand = scratch_return_to_number(builders, parsed_left, function);
                ScratchReturnTypes::Bool(
                    builders
                        .builder
                        .build_float_compare(FloatPredicate::OGT, left_hand, right_hand, "gt")
                        .unwrap(),
                )
            }
        }
        _ => todo!("あとでやる"),
    }
}
