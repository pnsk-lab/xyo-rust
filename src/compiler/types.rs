use std::collections::HashMap;

use inkwell::{
    AddressSpace,
    builder::Builder,
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    types::StructType,
    values::{FunctionValue, PointerValue},
};

use crate::{
    compiler::utils::{build_xor_shift_128_plus, gen_nbit_prime},
    jit::math_host_addresses,
    types::{ScratchProject, StageOrSprite},
};

include!(concat!(env!("OUT_DIR"), "/embedded_bitcodes.rs"));

#[repr(C)]
pub struct StringStruct {
    pub length: u64,
    pub container: *mut u16,
    pub hash1: u64,
    pub hash2: u64,
}
pub fn create_string_struct_type<'a>(context: &'a Context) -> StructType<'a> {
    context.struct_type(
        &[
            context.i64_type().into(),
            context.ptr_type(AddressSpace::default()).into(),
            context.i64_type().into(),
            context.i64_type().into(),
        ],
        false,
    )
}
pub enum StringKeys {
    Container,
    Length,
    Hash1,
    Hash2,
}
impl From<StringKeys> for u32 {
    fn from(field: StringKeys) -> Self {
        match field {
            StringKeys::Length => 0,
            StringKeys::Container => 1,
            StringKeys::Hash1 => 2,
            StringKeys::Hash2 => 3,
        }
    }
}

#[repr(C)]
pub struct SpriteStruct {
    pub sprite_x: f64,
    pub sprite_y: f64,
    pub sprite_rotate: f64,
}
pub fn create_sprite_struct_type<'a>(context: &'a Context) -> StructType<'a> {
    context.struct_type(
        &[
            context.f64_type().into(),
            context.f64_type().into(),
            context.f64_type().into(),
        ],
        false,
    )
}
pub enum SpriteKeys {
    SpriteX,
    SpriteY,
    SpriteRotate,
}
impl From<SpriteKeys> for u32 {
    fn from(field: SpriteKeys) -> Self {
        match field {
            SpriteKeys::SpriteX => 0,
            SpriteKeys::SpriteY => 1,
            SpriteKeys::SpriteRotate => 2,
        }
    }
}

pub struct Builders<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub global_variables: HashMap<String, usize>,
    pub local_variables: HashMap<usize, HashMap<String, usize>>,
    local_variables_increments: HashMap<usize, usize>,
    global_variable_increment: usize,
    counter: usize,
    pub functions: Functions<'ctx>,
    pub rolling_hash_seed_1: u64,
    pub rolling_hash_seed_2: u64,
    pub rolling_hash_base_1: u64,
    pub rolling_hash_base_2: u64,
    pub string_literals: HashMap<String, PointerValue<'ctx>>,
}
pub struct Functions<'ctx> {
    pub math_abs: FunctionValue<'ctx>,
    pub math_floor: FunctionValue<'ctx>,
    pub math_ceil: FunctionValue<'ctx>,
    pub math_sqrt: FunctionValue<'ctx>,
    pub math_sin: FunctionValue<'ctx>,
    pub math_cos: FunctionValue<'ctx>,
    pub math_tan: FunctionValue<'ctx>,
    pub math_asin: FunctionValue<'ctx>,
    pub math_acos: FunctionValue<'ctx>,
    pub math_atan: FunctionValue<'ctx>,
    pub math_loge: FunctionValue<'ctx>,
    pub math_log10: FunctionValue<'ctx>,
    pub math_exp: FunctionValue<'ctx>,
    pub math_pow10: FunctionValue<'ctx>,
    pub str_to_num: FunctionValue<'ctx>,
    pub num_to_str: FunctionValue<'ctx>,
    pub str_cmp_gt: FunctionValue<'ctx>,
    pub str_cmp_lt: FunctionValue<'ctx>,
    pub str_cmp_eq: FunctionValue<'ctx>,
    pub str_to_bool: FunctionValue<'ctx>,
    pub is_num: FunctionValue<'ctx>,
    pub rand: FunctionValue<'ctx>,
}
pub struct VariableInfo {
    is_global: bool,
    variable_idx: usize,
}

fn add_unary_f64_to_f64<'a>(
    module: &Module<'a>,
    context: &'a Context,
    name: &str,
    host_address: usize,
) -> FunctionValue<'a> {
    let fn_type = context
        .f64_type()
        .fn_type(&[context.f64_type().into()], false);
    let function = module.add_function(name, fn_type, None);
    let entry = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    let function_pointer = context
        .i64_type()
        .const_int(host_address as u64, false)
        .const_to_pointer(context.ptr_type(AddressSpace::default()));

    builder.position_at_end(entry);

    let argument = function.get_first_param().unwrap();
    let result = builder
        .build_indirect_call(fn_type, function_pointer, &[argument.into()], "host_call")
        .unwrap()
        .try_as_basic_value()
        .unwrap_basic()
        .into_float_value();
    builder.build_return(Some(&result)).unwrap();

    function
}

impl<'ctx> Builders<'ctx> {
    pub fn new(context: &'ctx Context, project: &ScratchProject) -> Self {
        let module = context.create_module("xyojit");
        let builder = context.create_builder();
        let ptr_type = context.ptr_type(AddressSpace::default());
        let f64_type = context.f64_type();
        let i64_type = context.i64_type();
        let i1_type = context.bool_type();
        let str_to_num_func_type = f64_type.fn_type(&[ptr_type.into()], false);
        let str_is_num_func_type = i1_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let num_to_str_func_type = ptr_type.fn_type(
            &[
                f64_type.into(),
                i64_type.into(),
                i64_type.into(),
                i64_type.into(),
                i64_type.into(),
            ],
            false,
        );
        let str_cmp_gt = i1_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let str_cmp_lt = i1_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let str_cmp_eq = i1_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let str_to_bool = i1_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let host_math = math_host_addresses();
        let functions = Functions {
            math_floor: add_unary_f64_to_f64(&module, &context, "xyo_floor", host_math.floor),
            math_abs: add_unary_f64_to_f64(&module, &context, "xyo_abs", host_math.abs),
            math_ceil: add_unary_f64_to_f64(&module, &context, "xyo_ceil", host_math.ceil),
            math_sqrt: add_unary_f64_to_f64(&module, &context, "xyo_sqrt", host_math.sqrt),
            math_sin: add_unary_f64_to_f64(&module, &context, "xyo_sin", host_math.sin),
            math_cos: add_unary_f64_to_f64(&module, &context, "xyo_cos", host_math.cos),
            math_tan: add_unary_f64_to_f64(&module, &context, "xyo_tan", host_math.tan),
            math_asin: add_unary_f64_to_f64(&module, &context, "xyo_asin", host_math.asin),
            math_acos: add_unary_f64_to_f64(&module, &context, "xyo_acos", host_math.acos),
            math_atan: add_unary_f64_to_f64(&module, &context, "xyo_atan", host_math.atan),
            math_loge: add_unary_f64_to_f64(&module, &context, "xyo_loge", host_math.loge),
            math_log10: add_unary_f64_to_f64(&module, &context, "xyo_log10", host_math.log10),
            math_exp: add_unary_f64_to_f64(&module, &context, "xyo_exp", host_math.exp),
            math_pow10: add_unary_f64_to_f64(&module, &context, "xyo_pow10", host_math.pow10),
            str_to_num: module.add_function("xyo_atod", str_to_num_func_type, None),
            num_to_str: module.add_function("xyo_dtoa", num_to_str_func_type, None),
            str_cmp_gt: module.add_function("xyo_str_cmp_gt", str_cmp_gt, None),
            str_cmp_lt: module.add_function("xyo_str_cmp_lt", str_cmp_lt, None),
            str_cmp_eq: module.add_function("xyo_str_cmp_eq", str_cmp_eq, None),
            is_num: module.add_function("str_is_num", str_is_num_func_type, None),
            str_to_bool: module.add_function("str_to_bool", str_to_bool, None),
            rand: build_xor_shift_128_plus(&context, &module),
        };
        let hash_seed_1 = gen_nbit_prime(64);
        let hash_seed_2 = gen_nbit_prime(64);
        let hash_base_1 = gen_nbit_prime(17);
        let hash_base_2 = gen_nbit_prime(17);
        let (
            global_variables,
            local_variables,
            global_variable_increment,
            local_variables_increments,
        ) = Self::create_variable_map(project);
        Self::link_generated_bitcodes(&module, context);
        Self {
            context,
            module,
            builder,
            counter: 0,
            functions,
            global_variables,
            local_variables,
            global_variable_increment,
            local_variables_increments,
            rolling_hash_seed_1: hash_seed_1,
            rolling_hash_seed_2: hash_seed_2,
            rolling_hash_base_1: hash_base_1,
            rolling_hash_base_2: hash_base_2,
            string_literals: HashMap::new(),
        }
    }
    fn link_generated_bitcodes(module: &Module<'ctx>, context: &'ctx Context) {
        for (name, bytes) in EMBEDDED_BITCODES {
            let buffer = MemoryBuffer::create_from_memory_range_copy(bytes, name);
            let bitcode_module = Module::parse_bitcode_from_buffer(&buffer, context)
                .unwrap_or_else(|err| panic!("failed to parse embedded {name}: {err}"));
            module
                .link_in_module(bitcode_module)
                .unwrap_or_else(|err| panic!("failed to link embedded {name}: {err}"));
        }
    }
    fn create_variable_map(
        project: &ScratchProject,
    ) -> (
        HashMap<String, usize>,
        HashMap<usize, HashMap<String, usize>>,
        usize,
        HashMap<usize, usize>,
    ) {
        let targets = &project.targets;
        let mut local_variable: HashMap<usize, HashMap<String, usize>> = HashMap::new();
        let mut global_variable: HashMap<String, usize> = HashMap::new();
        let mut global_variables_increment: usize = 0;
        let mut local_variables_increments: HashMap<usize, usize> = HashMap::new();
        for (target_idx, target) in targets.iter().enumerate() {
            match target {
                StageOrSprite::Stage(v) => {
                    for i in &v.variables {
                        global_variable.insert(i.0.clone(), global_variables_increment);
                        global_variables_increment += 1;
                    }
                }
                StageOrSprite::Sprite(v) => {
                    let mut counter: usize = 0;
                    let mut local_variable_temp: HashMap<String, usize> = HashMap::new();
                    for i in &v.variables {
                        local_variable_temp.insert(i.0.clone(), counter);
                        counter += 1;
                    }
                    local_variable.insert(target_idx, local_variable_temp);
                    local_variables_increments.insert(target_idx, counter);
                }
            }
        }
        (
            global_variable,
            local_variable,
            global_variables_increment,
            local_variables_increments,
        )
    }
    pub fn get_variable(&self, target_idx: usize, variable_id: &str) -> Option<VariableInfo> {
        if let Some(idx) = self.global_variables.get(variable_id) {
            return Some(VariableInfo {
                is_global: true,
                variable_idx: *idx,
            });
        }
        if let Some(local_variable_map) = self.local_variables.get(&target_idx)
            && local_variable_map.contains_key(variable_id)
        {
            return Some(VariableInfo {
                is_global: false,
                variable_idx: *local_variable_map.get(variable_id).unwrap(),
            });
        }
        None
    }
    pub fn create_function_name(&mut self) -> String {
        let mut n = self.counter;
        if n == 0 {
            self.counter += 1;
            return "func_a".to_string();
        }

        let mut chars = Vec::new();

        while n > 0 {
            let idx = (n % 26) as u8;
            chars.push((b'a' + idx) as char);
            n /= 26;
        }

        self.counter += 1;

        format!("func_{}", chars.iter().rev().collect::<String>())
    }
}
