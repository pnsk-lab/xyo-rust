use inkwell::{
    AddressSpace, FloatPredicate,
    context::Context,
    module::Module,
    values::{FloatValue, FunctionValue, IntValue},
};
use rand::Rng;

use crate::compiler::{compiler::ScratchReturnTypes, types::Builders};

// 整数か判定する
pub fn is_num<'ctx>(
    builders: &Builders<'ctx>,
    from: ScratchReturnTypes<'ctx>,
    _: &FunctionValue<'ctx>,
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

        ScratchReturnTypes::Bool(_) | ScratchReturnTypes::BoolLiteral(_) => {
            bool_t.const_int(1, false)
        }

        ScratchReturnTypes::String(v) => builders
            .builder
            .build_call(builders.functions.is_num, &[v.into()], "is_num")
            .unwrap()
            .try_as_basic_value()
            .basic()
            .unwrap()
            .into_int_value(),

        ScratchReturnTypes::NumberLiteral(v) => {
            let result = if v.is_nan() { true } else { v.floor() == v };
            bool_t.const_int(result as u64, false)
        }

        ScratchReturnTypes::StringLiteral(v) => {
            bool_t.const_int((!v.0.contains('.')) as u64, false)
        }
    }
}

pub fn scratch_return_to_number<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    func: &FunctionValue<'ctx>,
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
        ScratchReturnTypes::String(v) => {
            let p = func.get_first_param().unwrap().into_pointer_value();
            builders
                .builder
                .build_call(
                    builders.functions.str_to_num,
                    &[p.into(), (*v).into()],
                    "xyo_str_to_num",
                )
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value()
        }
        ScratchReturnTypes::NumberLiteral(v) => builders.context.f64_type().const_float(*v),
        ScratchReturnTypes::StringLiteral(v) => builders
            .context
            .f64_type()
            .const_float(v.0.parse::<f64>().unwrap_or(f64::NAN)),
        ScratchReturnTypes::BoolLiteral(v) => {
            builders.context.f64_type().const_float(v.0 as u8 as f64)
        }
    }
}
pub fn scratch_return_to_string<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    func: &FunctionValue<'ctx>,
    strings: &mut Vec<String>,
) -> inkwell::values::IntValue<'ctx> {
    match from {
        ScratchReturnTypes::Number(v) => builders
            .builder
            .build_call(
                builders.functions.num_to_str,
                &[func.get_first_param().unwrap().into(), (*v).into()],
                "xyo_num_to_str",
            )
            .unwrap()
            .try_as_basic_value()
            .basic()
            .unwrap()
            .into_int_value(),
        ScratchReturnTypes::Bool(v) => builders
            .builder
            .build_call(
                builders.functions.bool_to_str,
                &[func.get_first_param().unwrap().into(), (*v).into()],
                "xyo_bool_to_str",
            )
            .unwrap()
            .try_as_basic_value()
            .basic()
            .unwrap()
            .into_int_value(),
        ScratchReturnTypes::String(v) => *v,
        ScratchReturnTypes::NumberLiteral(v) => {
            let s = v.to_string();
            let idx = strings.len() as u64;
            strings.push(s);
            builders.context.i64_type().const_int(idx, false)
        }
        ScratchReturnTypes::StringLiteral(v) => v.1,
        ScratchReturnTypes::BoolLiteral(v) => {
            let s = v.0.to_string();
            let idx = strings.len() as u64;
            strings.push(s);
            builders.context.i64_type().const_int(idx, false)
        }
    }
}

pub fn build_xor_shift_128_plus<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
) -> FunctionValue<'ctx> {
    let i64_type = context.i64_type();

    let state0 = module.add_global(
        i64_type,
        Some(AddressSpace::default()),
        "xorshift128_state_0",
    );
    let state1 = module.add_global(
        i64_type,
        Some(AddressSpace::default()),
        "xorshift128_state_1",
    );

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
