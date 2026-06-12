use inkwell::{
    AddressSpace, FloatPredicate, IntPredicate,
    basic_block::BasicBlock,
    context::Context,
    module::Module,
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};
use rand::Rng;
use ryu_js::Buffer;

use crate::compiler::{
    compiler::ScratchReturnTypes,
    types::{Builders, DynamicKind, create_dynamic_struct_type, create_string_struct_type},
};

struct DynamicParts<'ctx> {
    dispatch_block: BasicBlock<'ctx>,
    kind: IntValue<'ctx>,
    payload: PointerValue<'ctx>,
}

fn dynamic_kind<'ctx>(builders: &Builders<'ctx>, kind: DynamicKind) -> IntValue<'ctx> {
    builders.context.i8_type().const_int((kind as u8).into(), false)
}

fn enter_dynamic_dispatch<'ctx>(
    builders: &Builders<'ctx>,
    dynamic: PointerValue<'ctx>,
    function: &FunctionValue<'ctx>,
) -> DynamicParts<'ctx> {
    let dispatch_block = builders.context.append_basic_block(*function, "dynamic_dispatch");
    builders.builder.build_unconditional_branch(dispatch_block).unwrap();
    builders.builder.position_at_end(dispatch_block);

    let dynamic_struct = create_dynamic_struct_type(builders.context);
    let kind_ptr = builders
        .builder
        .build_struct_gep(dynamic_struct, dynamic, 0, "dynamic_kind_ptr")
        .unwrap();
    let kind = builders
        .builder
        .build_load(builders.context.i8_type(), kind_ptr, "dynamic_kind")
        .unwrap()
        .into_int_value();
    let payload_slot = builders
        .builder
        .build_struct_gep(dynamic_struct, dynamic, 1, "dynamic_payload_slot")
        .unwrap();
    let payload = builders
        .builder
        .build_load(
            builders.context.ptr_type(AddressSpace::default()),
            payload_slot,
            "dynamic_payload",
        )
        .unwrap()
        .into_pointer_value();

    DynamicParts {
        dispatch_block,
        kind,
        payload,
    }
}

fn build_number_to_string<'ctx>(builders: &Builders<'ctx>, value: FloatValue<'ctx>) -> PointerValue<'ctx> {
    builders
        .builder
        .build_call(
            builders.functions.num_to_str,
            &[
                value.into(),
                builders
                    .context
                    .i64_type()
                    .const_int(builders.rolling_hash_seed_1, false)
                    .into(),
                builders
                    .context
                    .i64_type()
                    .const_int(builders.rolling_hash_base_1, false)
                    .into(),
                builders
                    .context
                    .i64_type()
                    .const_int(builders.rolling_hash_seed_2, false)
                    .into(),
                builders
                    .context
                    .i64_type()
                    .const_int(builders.rolling_hash_base_2, false)
                    .into(),
            ],
            "xyo_num_to_str",
        )
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_pointer_value()
}

fn build_bool_to_string<'ctx>(builders: &Builders<'ctx>, value: IntValue<'ctx>) -> PointerValue<'ctx> {
    builders
        .builder
        .build_select(
            builders
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    value,
                    builders.context.bool_type().const_int(1, false),
                    "is_true",
                )
                .unwrap(),
            create_string_struct(builders, "true"),
            create_string_struct(builders, "false"),
            "bool_to_str",
        )
        .unwrap()
        .into_pointer_value()
}

fn build_number_to_bool<'ctx>(builders: &Builders<'ctx>, value: FloatValue<'ctx>) -> IntValue<'ctx> {
    builders
        .builder
        .build_select(
            builders
                .builder
                .build_float_compare(
                    FloatPredicate::OEQ,
                    value,
                    builders.context.f64_type().const_float(0.0),
                    "is_zero",
                )
                .unwrap(),
            builders.context.bool_type().const_int(0, false),
            builders.context.bool_type().const_int(1, false),
            "number_to_bool",
        )
        .unwrap()
        .into_int_value()
}

fn build_string_to_bool<'ctx>(
    builders: &Builders<'ctx>,
    function: &FunctionValue<'ctx>,
    value: PointerValue<'ctx>,
) -> IntValue<'ctx> {
    builders
        .builder
        .build_call(
            builders.functions.str_to_bool,
            &[function.get_first_param().unwrap().into(), value.into()],
            "str_to_bool",
        )
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_int_value()
}

// 整数か判定する
pub fn is_num<'ctx>(
    builders: &Builders<'ctx>,
    from: ScratchReturnTypes<'ctx>,
    function: &FunctionValue<'ctx>,
) -> IntValue<'ctx> {
    let bool_t = builders.context.bool_type();

    match from {
        ScratchReturnTypes::Number(v) => {
            let builder = &builders.builder;

            let floor = builder
                .build_call(builders.functions.llvm_floor, &[v.into()], "floor")
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value();

            let is_int = builder
                .build_float_compare(FloatPredicate::OEQ, floor, v, "is_int")
                .unwrap();

            let is_nan = builder
                .build_float_compare(FloatPredicate::UNO, v, v, "is_nan")
                .unwrap();

            // NaN || floor(v) == v
            builder.build_or(is_nan, is_int, "is_num").unwrap()
        }

        ScratchReturnTypes::Bool(_) | ScratchReturnTypes::BoolLiteral(_) => bool_t.const_int(1, false),

        ScratchReturnTypes::String(v) => builders
            .builder
            .build_call(builders.functions.is_num, &[v.into()], "is_num")
            .unwrap()
            .try_as_basic_value()
            .basic()
            .unwrap()
            .into_int_value(),

        ScratchReturnTypes::NumberLiteral((v, _)) => {
            let result = if v.is_nan() { true } else { v.floor() == v };
            bool_t.const_int(result as u64, false)
        }

        ScratchReturnTypes::StringLiteral(v) => bool_t.const_int((!v.0.contains('.')) as u64, false),

        ScratchReturnTypes::Dynamic(v) => {
            let dynamic = enter_dynamic_dispatch(builders, v, function);
            let false_value = builders.context.bool_type().const_int(0, false);
            let number_block = builders.context.append_basic_block(*function, "double");
            let string_block = builders.context.append_basic_block(*function, "string");
            let finally = builders.context.append_basic_block(*function, "finally");
            builders
                .builder
                .build_switch(
                    dynamic.kind,
                    finally,
                    &[
                        (dynamic_kind(builders, DynamicKind::Number), number_block),
                        (dynamic_kind(builders, DynamicKind::String), string_block),
                    ],
                )
                .unwrap();
            builders.builder.position_at_end(number_block);
            let num = builders
                .builder
                .build_load(builders.context.f64_type(), dynamic.payload, "float")
                .unwrap()
                .into_float_value();
            let floor = builders
                .builder
                .build_call(builders.functions.llvm_floor, &[num.into()], "floor")
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value();

            let is_int = builders
                .builder
                .build_float_compare(FloatPredicate::OEQ, floor, num, "is_int")
                .unwrap();

            let is_nan = builders
                .builder
                .build_float_compare(FloatPredicate::UNO, num, num, "is_nan")
                .unwrap();

            let number_ret = builders.builder.build_or(is_nan, is_int, "is_num").unwrap();
            builders.builder.build_unconditional_branch(finally).unwrap();
            builders.builder.position_at_end(string_block);
            let string_ret = builders
                .builder
                .build_call(builders.functions.is_num, &[dynamic.payload.into()], "is_num")
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_int_value();
            builders.builder.build_unconditional_branch(finally).unwrap();
            builders.builder.position_at_end(finally);
            let phi = builders.builder.build_phi(builders.context.bool_type(), "phi").unwrap();
            phi.add_incoming(&[
                (&false_value, dynamic.dispatch_block),
                (&number_ret, number_block),
                (&string_ret, string_block),
            ]);
            phi.as_basic_value().into_int_value()
        }
    }
}
fn build_reusable_dynamic<'ctx, BuildNew, ReuseOld>(
    builders: &Builders<'ctx>,
    function: &FunctionValue<'ctx>,
    current_dynamic: Option<PointerValue<'ctx>>,
    current_kind: Option<IntValue<'ctx>>,
    expected_kind: DynamicKind,
    build_new: BuildNew,
    reuse_old: ReuseOld,
) -> PointerValue<'ctx>
where
    BuildNew: FnOnce() -> PointerValue<'ctx>,
    ReuseOld: FnOnce(PointerValue<'ctx>),
{
    let Some(current_dynamic) = current_dynamic else {
        return build_new();
    };
    let Some(current_kind) = current_kind else {
        return build_new();
    };

    let new_value_block = builders.context.append_basic_block(*function, "new_value");
    let use_old_value_block = builders.context.append_basic_block(*function, "use_old_value");
    let finally_block = builders.context.append_basic_block(*function, "finally");
    builders
        .builder
        .build_conditional_branch(
            builders
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    current_kind,
                    builders.context.i8_type().const_int(expected_kind as u64, false),
                    "dynamic_kind_matches",
                )
                .unwrap(),
            use_old_value_block,
            new_value_block,
        )
        .unwrap();

    builders.builder.position_at_end(new_value_block);
    let new_dynamic = build_new();
    builders.builder.build_unconditional_branch(finally_block).unwrap();

    builders.builder.position_at_end(use_old_value_block);
    reuse_old(current_dynamic);
    builders.builder.build_unconditional_branch(finally_block).unwrap();

    builders.builder.position_at_end(finally_block);
    let phi = builders
        .builder
        .build_phi(builders.context.ptr_type(AddressSpace::default()), "dynamic")
        .unwrap();
    phi.add_incoming(&[(&new_dynamic, new_value_block), (&current_dynamic, use_old_value_block)]);
    phi.as_basic_value().into_pointer_value()
}

pub fn scratch_return_to_dynamic<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    function: &FunctionValue<'ctx>,
    from_v: Option<PointerValue<'ctx>>,
) -> PointerValue<'ctx> {
    let struct_ty = create_dynamic_struct_type(builders.context);
    let current_dynamic = from_v.map(|from_v| {
        builders
            .builder
            .build_load(
                builders.context.ptr_type(AddressSpace::default()),
                from_v,
                "current_dynamic",
            )
            .unwrap()
            .into_pointer_value()
    });
    let current_kind = current_dynamic.map(|current_dynamic| {
        builders
            .builder
            .build_load(
                builders.context.i8_type(),
                builders
                    .builder
                    .build_struct_gep(struct_ty, current_dynamic, 0, "current_dynamic_kind_ptr")
                    .unwrap(),
                "current_dynamic_kind",
            )
            .unwrap()
            .into_int_value()
    });
    match from {
        ScratchReturnTypes::Bool(v) | ScratchReturnTypes::BoolLiteral((_, v)) => build_reusable_dynamic(
            builders,
            function,
            current_dynamic,
            current_kind,
            DynamicKind::Bool,
            || {
                let bool_ptr = builders
                    .builder
                    .build_malloc(builders.context.bool_type(), "bool_ptr")
                    .unwrap();
                builders.builder.build_store(bool_ptr, *v).unwrap();
                let struct_ptr = builders.builder.build_malloc(struct_ty, "malloc").unwrap();
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, struct_ptr, 0, "type")
                            .unwrap(),
                        builders.context.i8_type().const_int(DynamicKind::Bool as u64, false),
                    )
                    .unwrap();
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, struct_ptr, 1, "ptr")
                            .unwrap(),
                        bool_ptr,
                    )
                    .unwrap();
                struct_ptr
            },
            |old_struct_ptr| {
                let old_bool_ptr = builders
                    .builder
                    .build_load(
                        builders.context.ptr_type(AddressSpace::default()),
                        builders
                            .builder
                            .build_struct_gep(struct_ty, old_struct_ptr, 1, "old_bool_ptr_slot")
                            .unwrap(),
                        "old_bool_ptr",
                    )
                    .unwrap()
                    .into_pointer_value();
                builders.builder.build_store(old_bool_ptr, *v).unwrap();
            },
        ),
        ScratchReturnTypes::Number(v) | ScratchReturnTypes::NumberLiteral((_, v)) => build_reusable_dynamic(
            builders,
            function,
            current_dynamic,
            current_kind,
            DynamicKind::Number,
            || {
                let number_ptr = builders
                    .builder
                    .build_malloc(builders.context.f64_type(), "number_ptr")
                    .unwrap();
                builders.builder.build_store(number_ptr, *v).unwrap();
                let struct_ptr = builders.builder.build_malloc(struct_ty, "malloc").unwrap();
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, struct_ptr, 0, "type")
                            .unwrap(),
                        builders.context.i8_type().const_int(DynamicKind::Number as u64, false),
                    )
                    .unwrap();
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, struct_ptr, 1, "ptr")
                            .unwrap(),
                        number_ptr,
                    )
                    .unwrap();
                struct_ptr
            },
            |old_struct_ptr| {
                let old_number_ptr = builders
                    .builder
                    .build_load(
                        builders.context.ptr_type(AddressSpace::default()),
                        builders
                            .builder
                            .build_struct_gep(struct_ty, old_struct_ptr, 1, "old_number_ptr_slot")
                            .unwrap(),
                        "old_number_ptr",
                    )
                    .unwrap()
                    .into_pointer_value();
                builders.builder.build_store(old_number_ptr, *v).unwrap();
            },
        ),
        ScratchReturnTypes::String(v) | ScratchReturnTypes::StringLiteral((_, v)) => build_reusable_dynamic(
            builders,
            function,
            current_dynamic,
            current_kind,
            DynamicKind::String,
            || {
                let struct_ptr = builders.builder.build_malloc(struct_ty, "malloc").unwrap();
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, struct_ptr, 0, "type")
                            .unwrap(),
                        builders.context.i8_type().const_int(DynamicKind::String as u64, false),
                    )
                    .unwrap();
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, struct_ptr, 1, "ptr")
                            .unwrap(),
                        *v,
                    )
                    .unwrap();
                struct_ptr
            },
            |old_struct_ptr| {
                builders
                    .builder
                    .build_store(
                        builders
                            .builder
                            .build_struct_gep(struct_ty, old_struct_ptr, 1, "old_string_ptr_slot")
                            .unwrap(),
                        *v,
                    )
                    .unwrap();
            },
        ),
        ScratchReturnTypes::Dynamic(v) => *v,
    }
}
pub fn scratch_return_to_number<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    function: &FunctionValue<'ctx>,
) -> FloatValue<'ctx> {
    match from {
        ScratchReturnTypes::Number(v) => *v,
        ScratchReturnTypes::Bool(v) => builders
            .builder
            .build_select(
                *v,
                builders.context.f64_type().const_float(1.0),
                builders.context.f64_type().const_float(0.0),
                "num_bool",
            )
            .unwrap()
            .into_float_value(),
        ScratchReturnTypes::String(v) => builders
            .builder
            .build_call(builders.functions.str_to_num, &[(*v).into()], "xyo_atod")
            .unwrap()
            .try_as_basic_value()
            .basic()
            .unwrap()
            .into_float_value(),
        ScratchReturnTypes::NumberLiteral((_, v)) => *v,
        ScratchReturnTypes::StringLiteral(v) => builders
            .context
            .f64_type()
            .const_float(v.0.parse::<f64>().unwrap_or(f64::NAN)),
        ScratchReturnTypes::BoolLiteral(v) => builders.context.f64_type().const_float(v.0 as u8 as f64),
        ScratchReturnTypes::Dynamic(v) => {
            let dynamic = enter_dynamic_dispatch(builders, *v, function);
            let number_block = builders.context.append_basic_block(*function, "double");
            let string_block = builders.context.append_basic_block(*function, "string");
            let boolean_block = builders.context.append_basic_block(*function, "boolean");
            let finally = builders.context.append_basic_block(*function, "finally");
            builders
                .builder
                .build_switch(
                    dynamic.kind,
                    finally,
                    &[
                        (dynamic_kind(builders, DynamicKind::Number), number_block),
                        (dynamic_kind(builders, DynamicKind::Bool), boolean_block),
                        (dynamic_kind(builders, DynamicKind::String), string_block),
                    ],
                )
                .unwrap();
            builders.builder.position_at_end(number_block);
            let number_ret = builders
                .builder
                .build_load(builders.context.f64_type(), dynamic.payload, "float")
                .unwrap()
                .into_float_value();
            builders.builder.build_unconditional_branch(finally).unwrap();
            builders.builder.position_at_end(string_block);
            let string_ret = builders
                .builder
                .build_call(builders.functions.str_to_num, &[dynamic.payload.into()], "xyo_atod")
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value();
            builders.builder.build_unconditional_branch(finally).unwrap();
            builders.builder.position_at_end(boolean_block);
            let bool_value = builders
                .builder
                .build_load(builders.context.bool_type(), dynamic.payload, "bool")
                .unwrap()
                .into_int_value();
            let bool_ret = builders
                .builder
                .build_select(
                    bool_value,
                    builders.context.f64_type().const_float(1.0),
                    builders.context.f64_type().const_float(0.0),
                    "num_bool",
                )
                .unwrap()
                .into_float_value();
            builders.builder.build_unconditional_branch(finally).unwrap();
            builders.builder.position_at_end(finally);
            let phi = builders.builder.build_phi(builders.context.f64_type(), "phi").unwrap();
            phi.add_incoming(&[
                (
                    &builders.context.f64_type().const_float(f64::NAN),
                    dynamic.dispatch_block,
                ),
                (&number_ret, number_block),
                (&string_ret, string_block),
                (&bool_ret, boolean_block),
            ]);
            phi.as_basic_value().into_float_value()
        }
    }
}
pub fn js_number_to_string(x: f64) -> String {
    let mut buf = Buffer::new();
    buf.format(x).to_owned()
}
pub fn scratch_return_to_string<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    function: &FunctionValue<'ctx>,
) -> PointerValue<'ctx> {
    match from {
        ScratchReturnTypes::Number(v) => build_number_to_string(builders, *v),
        ScratchReturnTypes::Bool(v) => build_bool_to_string(builders, *v),
        ScratchReturnTypes::String(v) => *v,
        ScratchReturnTypes::NumberLiteral((v, _)) => create_string_struct(builders, &js_number_to_string(*v)),
        ScratchReturnTypes::StringLiteral(v) => v.1,
        ScratchReturnTypes::BoolLiteral(v) => create_string_struct(builders, &v.0.to_string()),
        ScratchReturnTypes::Dynamic(v) => {
            let dynamic = enter_dynamic_dispatch(builders, *v, function);
            let number_block = builders.context.append_basic_block(*function, "double");
            let string_block = builders.context.append_basic_block(*function, "string");
            let boolean_block = builders.context.append_basic_block(*function, "boolean");
            let finally = builders.context.append_basic_block(*function, "finally");
            builders
                .builder
                .build_switch(
                    dynamic.kind,
                    finally,
                    &[
                        (dynamic_kind(builders, DynamicKind::Number), number_block),
                        (dynamic_kind(builders, DynamicKind::Bool), boolean_block),
                        (dynamic_kind(builders, DynamicKind::String), string_block),
                    ],
                )
                .unwrap();

            builders.builder.position_at_end(number_block);
            let number_value = builders
                .builder
                .build_load(builders.context.f64_type(), dynamic.payload, "dynamic_number")
                .unwrap()
                .into_float_value();
            let number_ret = build_number_to_string(builders, number_value);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(string_block);
            let string_ret = dynamic.payload;
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(boolean_block);
            let bool_value = builders
                .builder
                .build_load(builders.context.bool_type(), dynamic.payload, "dynamic_bool")
                .unwrap()
                .into_int_value();
            let bool_ret = build_bool_to_string(builders, bool_value);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(finally);
            let default_ret = create_string_struct(builders, "");
            let phi = builders
                .builder
                .build_phi(builders.context.ptr_type(AddressSpace::default()), "dynamic_to_string")
                .unwrap();
            phi.add_incoming(&[
                (&default_ret, dynamic.dispatch_block),
                (&number_ret, number_block),
                (&string_ret, string_block),
                (&bool_ret, boolean_block),
            ]);
            phi.as_basic_value().into_pointer_value()
        }
    }
}

pub fn scratch_return_to_bool<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    func: &FunctionValue<'ctx>,
) -> inkwell::values::IntValue<'ctx> {
    match from {
        ScratchReturnTypes::Number(v) => build_number_to_bool(builders, *v),
        ScratchReturnTypes::Bool(v) => *v,
        ScratchReturnTypes::String(v) => build_string_to_bool(builders, func, *v),
        ScratchReturnTypes::NumberLiteral((v, _)) => builders.context.bool_type().const_int((*v != 0.0) as u64, false),
        ScratchReturnTypes::StringLiteral(v) => {
            let string_bool = match v.0.to_lowercase().as_str() {
                "" => false,
                "0" => false,
                "false" => false,
                _ => true,
            };
            builders.context.bool_type().const_int(string_bool as u64, false)
        }
        ScratchReturnTypes::BoolLiteral(v) => v.1,
        ScratchReturnTypes::Dynamic(v) => {
            let dynamic = enter_dynamic_dispatch(builders, *v, func);
            let false_value = builders.context.bool_type().const_int(0, false);
            let number_block = builders.context.append_basic_block(*func, "double");
            let string_block = builders.context.append_basic_block(*func, "string");
            let boolean_block = builders.context.append_basic_block(*func, "boolean");
            let finally = builders.context.append_basic_block(*func, "finally");
            builders
                .builder
                .build_switch(
                    dynamic.kind,
                    finally,
                    &[
                        (dynamic_kind(builders, DynamicKind::Number), number_block),
                        (dynamic_kind(builders, DynamicKind::Bool), boolean_block),
                        (dynamic_kind(builders, DynamicKind::String), string_block),
                    ],
                )
                .unwrap();

            builders.builder.position_at_end(number_block);
            let number_value = builders
                .builder
                .build_load(builders.context.f64_type(), dynamic.payload, "dynamic_number")
                .unwrap()
                .into_float_value();
            let number_ret = build_number_to_bool(builders, number_value);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(string_block);
            let string_ret = build_string_to_bool(builders, func, dynamic.payload);
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(boolean_block);
            let bool_ret = builders
                .builder
                .build_load(builders.context.bool_type(), dynamic.payload, "dynamic_bool")
                .unwrap()
                .into_int_value();
            builders.builder.build_unconditional_branch(finally).unwrap();

            builders.builder.position_at_end(finally);
            let phi = builders
                .builder
                .build_phi(builders.context.bool_type(), "dynamic_to_bool")
                .unwrap();
            phi.add_incoming(&[
                (&false_value, dynamic.dispatch_block),
                (&number_ret, number_block),
                (&string_ret, string_block),
                (&bool_ret, boolean_block),
            ]);
            phi.as_basic_value().into_int_value()
        }
    }
}

pub fn build_xor_shift_128_plus<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> FunctionValue<'ctx> {
    let i64_type = context.i64_type();

    let state0 = module.add_global(i64_type, Some(AddressSpace::default()), "xorshift128_state_0");
    let state1 = module.add_global(i64_type, Some(AddressSpace::default()), "xorshift128_state_1");

    let mut rng = rand::rng();
    let (s0, s1) = loop {
        let s0 = rng.next_u64();
        let s1 = rng.next_u64();
        if s0 != 0 || s1 != 0 {
            break (s0, s1);
        }
    };

    state0.set_initializer(&i64_type.const_int(s0, false));
    state1.set_initializer(&i64_type.const_int(s1, false));

    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function("xorshift128plus", fn_type, None);

    let entry = context.append_basic_block(function, "entry");
    let builder = context.create_builder();

    builder.position_at_end(entry);

    let x = builder
        .build_load(i64_type, state0.as_pointer_value(), "x")
        .unwrap()
        .into_int_value();

    let y = builder
        .build_load(i64_type, state1.as_pointer_value(), "y")
        .unwrap()
        .into_int_value();

    builder.build_store(state0.as_pointer_value(), y).unwrap();

    let x_shift = builder
        .build_left_shift(x, i64_type.const_int(23, false), "x_shift")
        .unwrap();

    let x2 = builder.build_xor(x, x_shift, "x2").unwrap();

    let x_shift2 = builder
        .build_right_shift(x2, i64_type.const_int(17, false), false, "x_shift2")
        .unwrap();

    let x3 = builder.build_xor(x2, x_shift2, "x3").unwrap();

    let x4 = builder.build_xor(x3, y, "x4").unwrap();

    let y_shift2 = builder
        .build_right_shift(y, i64_type.const_int(26, false), false, "y_shift2")
        .unwrap();

    let x5 = builder.build_xor(x4, y_shift2, "x5").unwrap();

    builder.build_store(state1.as_pointer_value(), x5).unwrap();

    let ret = builder.build_int_add(x5, y, "ret").unwrap();
    builder.build_return(Some(&ret)).unwrap();

    function
}

fn mod_pow(mut base: u128, mut exp: u128, modu: u128) -> u128 {
    let mut result = 1u128;
    base %= modu;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modu;
        }
        base = (base * base) % modu;
        exp >>= 1;
    }
    result
}

fn is_prime(n: u64) -> bool {
    const SMALL_PRIMES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    if n < 2 {
        return false;
    }
    for &p in &SMALL_PRIMES {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    const BASES: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

    'outer: for &a in &BASES {
        let a = a % n;
        if a == 0 {
            continue;
        }
        let mut x = mod_pow(a as u128, d as u128, n as u128) as u64;
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..(s - 1) {
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
pub fn gen_nbit_prime(n: usize) -> u64 {
    if n < 2 || n > 64 {
        panic!("n must be between 2 and 64");
    }
    loop {
        let mut rng = rand::rng();
        let num = rng.next_u64() | (1 << (n - 1)) | 1;
        if n == 64 {
            if is_prime(num) {
                return num;
            }
            continue;
        }
        let n = num % (1 << n);

        if is_prime(n) {
            return n;
        }
    }
}
pub fn calc_rolling_hash<'ctx>(s: &str, builders: &Builders<'ctx>) -> (u64, u64) {
    let base1 = builders.rolling_hash_base_1;
    let base2 = builders.rolling_hash_base_2;
    let mod1 = builders.rolling_hash_seed_1;
    let mod2 = builders.rolling_hash_seed_2;
    let mut hash1 = 0u64;
    let mut hash2 = 0u64;
    let utf16_str: Vec<u16> = s.encode_utf16().collect();
    for &c in &utf16_str {
        hash1 = (hash1.wrapping_mul(base1).wrapping_add(c as u64)) % mod1;
        hash2 = (hash2.wrapping_mul(base2).wrapping_add(c as u64)) % mod2;
    }
    (hash1, hash2)
}
pub fn create_string_struct<'ctx>(builders: &Builders<'ctx>, s: &str) -> PointerValue<'ctx> {
    if let Some(p) = builders.string_literals.get(&s.to_string()) {
        return *p;
    }
    let utf16_str: Vec<u16> = s.encode_utf16().collect();
    let length = utf16_str.len() as u64;
    let (hash1, hash2) = calc_rolling_hash(s, builders);
    let string_struct_type = create_string_struct_type(builders.context);
    let global = builders
        .module
        .add_global(string_struct_type, Some(AddressSpace::default()), "string_struct");
    let data_global = builders.module.add_global(
        builders.context.i16_type().array_type(length as u32),
        Some(AddressSpace::default()),
        "string_data",
    );
    let i16_type = builders.context.i16_type();

    let values: Vec<_> = utf16_str
        .iter()
        .map(|&v| i16_type.const_int(v as u64, false).into())
        .collect();

    data_global.set_initializer(&i16_type.const_array(&values));
    global.set_initializer(&string_struct_type.const_named_struct(&[
        builders.context.i64_type().const_int(length, false).into(),
        data_global.as_pointer_value().into(),
        builders.context.i64_type().const_int(hash1, false).into(),
        builders.context.i64_type().const_int(hash2, false).into(),
    ]));
    global.as_pointer_value()
}

#[cfg(test)]
mod tests {
    use inkwell::{AddressSpace, context::Context};
    use serde_json::json;

    use super::*;
    use crate::types::ScratchProject;

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

    fn test_function<'ctx>(context: &'ctx Context, builders: &Builders<'ctx>) -> FunctionValue<'ctx> {
        let fn_type = context
            .void_type()
            .fn_type(&[context.ptr_type(AddressSpace::default()).into()], false);
        let function = builders.module.add_function("dynamic_test_func", fn_type, None);
        let entry = context.append_basic_block(function, "entry");
        builders.builder.position_at_end(entry);
        function
    }

    fn create_dynamic<'ctx>(
        builders: &Builders<'ctx>,
        kind: DynamicKind,
        payload: PointerValue<'ctx>,
    ) -> PointerValue<'ctx> {
        let dynamic_struct = create_dynamic_struct_type(builders.context);
        let dynamic = builders.builder.build_alloca(dynamic_struct, "dynamic").unwrap();
        let kind_ptr = builders
            .builder
            .build_struct_gep(dynamic_struct, dynamic, 0, "dynamic_kind")
            .unwrap();
        builders
            .builder
            .build_store(kind_ptr, dynamic_kind(builders, kind))
            .unwrap();
        let payload_ptr = builders
            .builder
            .build_struct_gep(dynamic_struct, dynamic, 1, "dynamic_payload")
            .unwrap();
        builders.builder.build_store(payload_ptr, payload).unwrap();
        dynamic
    }

    #[test]
    fn dynamic_conversions_build_verifiable_ir() {
        let context = Context::create();
        let project = empty_project();
        let builders = Builders::new(&context, &project);
        let function = test_function(&context, &builders);

        let number_payload = builders
            .builder
            .build_alloca(builders.context.f64_type(), "number_payload")
            .unwrap();
        builders
            .builder
            .build_store(number_payload, builders.context.f64_type().const_float(3.0))
            .unwrap();
        let dynamic = create_dynamic(&builders, DynamicKind::Number, number_payload);

        let _ = is_num(&builders, ScratchReturnTypes::Dynamic(dynamic), &function);
        let _ = scratch_return_to_number(&builders, &ScratchReturnTypes::Dynamic(dynamic), &function);
        let _ = scratch_return_to_string(&builders, &ScratchReturnTypes::Dynamic(dynamic), &function);
        let _ = scratch_return_to_bool(&builders, &ScratchReturnTypes::Dynamic(dynamic), &function);
        builders.builder.build_return(None).unwrap();

        assert!(builders.module.verify().is_ok(), "{}", builders.module.to_string());
    }

    #[test]
    fn scratch_return_to_dynamic_reuses_matching_dynamic_payloads() {
        let context = Context::create();
        let project = empty_project();
        let builders = Builders::new(&context, &project);
        let function = test_function(&context, &builders);
        let ptr_type = builders.context.ptr_type(AddressSpace::default());

        let bool_payload = builders
            .builder
            .build_alloca(builders.context.bool_type(), "bool_payload")
            .unwrap();
        let bool_dynamic = create_dynamic(&builders, DynamicKind::Bool, bool_payload);
        let bool_slot = builders.builder.build_alloca(ptr_type, "bool_slot").unwrap();
        builders.builder.build_store(bool_slot, bool_dynamic).unwrap();
        let _ = scratch_return_to_dynamic(
            &builders,
            &ScratchReturnTypes::BoolLiteral((true, builders.context.bool_type().const_int(1, false))),
            &function,
            Some(bool_slot),
        );

        let number_payload = builders
            .builder
            .build_alloca(builders.context.f64_type(), "number_payload")
            .unwrap();
        let number_dynamic = create_dynamic(&builders, DynamicKind::Number, number_payload);
        let number_slot = builders.builder.build_alloca(ptr_type, "number_slot").unwrap();
        builders.builder.build_store(number_slot, number_dynamic).unwrap();
        let _ = scratch_return_to_dynamic(
            &builders,
            &ScratchReturnTypes::Number(builders.context.f64_type().const_float(42.0)),
            &function,
            Some(number_slot),
        );

        let string_dynamic = create_dynamic(&builders, DynamicKind::String, create_string_struct(&builders, "old"));
        let string_slot = builders.builder.build_alloca(ptr_type, "string_slot").unwrap();
        builders.builder.build_store(string_slot, string_dynamic).unwrap();
        let _ = scratch_return_to_dynamic(
            &builders,
            &ScratchReturnTypes::StringLiteral(("new".to_string(), create_string_struct(&builders, "new"))),
            &function,
            Some(string_slot),
        );

        builders.builder.build_return(None).unwrap();

        assert!(builders.module.verify().is_ok(), "{}", builders.module.to_string());
    }
}

#[cfg(windows)]
pub fn xyo_now_ns() -> i64 {
    use std::mem::MaybeUninit;

    #[link(name = "kernel32")]
    extern "system" {
        fn QueryPerformanceCounter(lp_performance_count: *mut i64) -> i32;
        fn QueryPerformanceFrequency(lp_frequency: *mut i64) -> i32;
    }

    unsafe {
        let mut counter = MaybeUninit::<i64>::uninit();
        let mut frequency = MaybeUninit::<i64>::uninit();

        let ok_counter = QueryPerformanceCounter(counter.as_mut_ptr());
        let ok_frequency = QueryPerformanceFrequency(frequency.as_mut_ptr());

        assert!(ok_counter != 0);
        assert!(ok_frequency != 0);

        let counter = counter.assume_init() as i128;
        let frequency = frequency.assume_init() as i128;

        ((counter * 1_000_000_000i128) / frequency) as i64
    }
}

#[cfg(not(windows))]
pub fn xyo_now_ns() -> i64 {
    use libc::{CLOCK_MONOTONIC, clock_gettime, timespec};
    use std::mem::MaybeUninit;

    unsafe {
        let mut ts = MaybeUninit::<timespec>::uninit();

        let ret = clock_gettime(CLOCK_MONOTONIC, ts.as_mut_ptr());
        assert_eq!(ret, 0);

        let ts = ts.assume_init();

        ts.tv_sec as i64 * 1_000_000_000i64 + ts.tv_nsec as i64
    }
}
