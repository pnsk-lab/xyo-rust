use inkwell::{
    AddressSpace, FloatPredicate, IntPredicate,
    context::Context,
    module::Module,
    values::{BasicValue, FloatValue, FunctionValue, IntValue, PointerValue},
};
use rand::Rng;
use ryu_js::Buffer;
use serde::de::value;

use crate::compiler::{
    compiler::ScratchReturnTypes,
    types::{Builders, create_string_struct_type},
};

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
                .build_call(builders.functions.math_floor, &[v.into()], "floor")
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
                .build_call(builders.functions.str_to_num, &[(*v).into()], "xyo_atod")
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
pub fn js_number_to_string(x: f64) -> String {
    let mut buf = Buffer::new();
    buf.format(x).to_owned()
}
pub fn scratch_return_to_string<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    func: &FunctionValue<'ctx>,
) -> PointerValue<'ctx> {
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
            .into_pointer_value(),
        ScratchReturnTypes::Bool(v) => builders
            .builder
            .build_select(
                builders
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        *v,
                        builders.context.bool_type().const_int(1, false),
                        "is_true",
                    )
                    .unwrap(),
                create_string_struct(builders, "true"),
                create_string_struct(builders, "false"),
                "bool_to_str",
            )
            .unwrap()
            .into_pointer_value(),
        ScratchReturnTypes::String(v) => *v,
        ScratchReturnTypes::NumberLiteral(v) => {
            create_string_struct(builders, &js_number_to_string(*v))
        }
        ScratchReturnTypes::StringLiteral(v) => v.1,
        ScratchReturnTypes::BoolLiteral(v) => create_string_struct(builders, &v.0.to_string()),
    }
}

pub fn scratch_return_to_bool<'ctx>(
    builders: &Builders<'ctx>,
    from: &ScratchReturnTypes<'ctx>,
    func: &FunctionValue<'ctx>,
) -> inkwell::values::IntValue<'ctx> {
    match from {
        ScratchReturnTypes::Number(v) => builders
            .builder
            .build_select(
                builders
                    .builder
                    .build_float_compare(
                        FloatPredicate::OEQ,
                        *v,
                        builders.context.f64_type().const_float(0.0),
                        "is_zero",
                    )
                    .unwrap(),
                builders.context.bool_type().const_int(0, false),
                builders.context.bool_type().const_int(1, false),
                "number_to_bool",
            )
            .unwrap()
            .into_int_value(),
        ScratchReturnTypes::Bool(v) => *v,
        ScratchReturnTypes::String(v) => builders
            .builder
            .build_call(
                builders.functions.str_to_bool,
                &[func.get_first_param().unwrap().into(), (*v).into()],
                "str_to_bool",
            )
            .unwrap()
            .try_as_basic_value()
            .basic()
            .unwrap()
            .into_int_value(),
        ScratchReturnTypes::NumberLiteral(v) => builders
            .context
            .bool_type()
            .const_int((*v != 0.0) as u64, false),
        ScratchReturnTypes::StringLiteral(v) => {
            let string_bool = match v.0.to_lowercase().as_str() {
                "" => false,
                "0" => false,
                "false" => false,
                _ => true,
            };
            builders
                .context
                .bool_type()
                .const_int(string_bool as u64, false)
        }
        ScratchReturnTypes::BoolLiteral(v) => v.1,
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
    let global = builders.module.add_global(
        string_struct_type,
        Some(AddressSpace::default()),
        "string_struct",
    );
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
