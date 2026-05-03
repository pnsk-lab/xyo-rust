use std::{
    borrow::Cow,
    collections::HashMap,
    ffi::{CStr, CString},
};

use inkwell::{
    AddressSpace,
    attributes::{Attribute, AttributeLoc},
    builder::Builder,
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    types::StructType,
    values::{FunctionValue, PointerValue},
};
use llvm_sys::core::LLVMCreateMemoryBufferWithMemoryRange;

use crate::{
    compiler::utils::{build_xor_shift_128_plus, gen_nbit_prime},
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
#[derive(Default, Debug)]
pub struct CostumeInfo {
    pub width: f64,
    pub height: f64,
}
pub fn create_costume_struct_type<'a>(context: &'a Context) -> StructType<'a> {
    context.struct_type(
        &[context.f64_type().into(), context.f64_type().into()],
        false,
    )
}
pub enum CostumeInfoKeys {
    Width,
    Height,
}
impl From<CostumeInfoKeys> for u32 {
    fn from(field: CostumeInfoKeys) -> Self {
        match field {
            CostumeInfoKeys::Width => 0,
            CostumeInfoKeys::Height => 1,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SpriteStruct {
    pub sprite_x: f64,
    pub sprite_y: f64,
    pub sprite_rotate: f64,
    pub sprite_size: f64,
    pub sprite_costume_id: i64,
    pub sprite_costumes: *mut CostumeInfo,
}

impl Default for SpriteStruct {
    fn default() -> Self {
        Self {
            sprite_x: 0.0,
            sprite_y: 0.0,
            sprite_rotate: 90.0,
            sprite_size: 100.0,
            sprite_costume_id: 0,
            sprite_costumes: std::ptr::null_mut(),
        }
    }
}
pub fn create_sprite_struct_type<'a>(context: &'a Context) -> StructType<'a> {
    context.struct_type(
        &[
            context.f64_type().into(),
            context.f64_type().into(),
            context.f64_type().into(),
            context.f64_type().into(),
            context.i64_type().into(),
            context.ptr_type(AddressSpace::default()).into(),
        ],
        false,
    )
}
pub enum SpriteKeys {
    SpriteX,
    SpriteY,
    SpriteRotate,
    SpriteSize,
    SpriteCostumeId,
    SpriteCostumes,
}
impl From<SpriteKeys> for u32 {
    fn from(field: SpriteKeys) -> Self {
        match field {
            SpriteKeys::SpriteX => 0,
            SpriteKeys::SpriteY => 1,
            SpriteKeys::SpriteRotate => 2,
            SpriteKeys::SpriteSize => 3,
            SpriteKeys::SpriteCostumeId => 4,
            SpriteKeys::SpriteCostumes => 5,
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
#[derive(Debug)]
pub struct Functions<'ctx> {
    pub llvm_abs: FunctionValue<'ctx>,
    pub llvm_floor: FunctionValue<'ctx>,
    pub llvm_ceil: FunctionValue<'ctx>,
    pub llvm_sqrt: FunctionValue<'ctx>,
    pub llvm_sin: FunctionValue<'ctx>,
    pub llvm_cos: FunctionValue<'ctx>,
    pub llvm_tan: FunctionValue<'ctx>,
    pub llvm_asin: FunctionValue<'ctx>,
    pub llvm_acos: FunctionValue<'ctx>,
    pub llvm_atan: FunctionValue<'ctx>,
    pub llvm_loge: FunctionValue<'ctx>,
    pub llvm_log10: FunctionValue<'ctx>,
    pub llvm_exp: FunctionValue<'ctx>,
    pub llvm_pow10: FunctionValue<'ctx>,
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

fn get_libm_f64_to_f64<'a>(
    module: &Module<'a>,
    context: &'a Context,
    name: &str,
) -> FunctionValue<'a> {
    let f64_type = context.f64_type();
    let floor_type = f64_type.fn_type(&[f64_type.into()], false);
    let func = module.add_function(name, floor_type, None);
    let nobuiltin_kind = Attribute::get_named_enum_kind_id("nobuiltin");
    let nobuiltin_attr = context.create_enum_attribute(nobuiltin_kind, 0);
    func.add_attribute(AttributeLoc::Function, nobuiltin_attr);
    func
}

#[inline]
pub(crate) fn to_c_str(mut s: &str) -> Cow<'_, CStr> {
    if s.is_empty() {
        s = "\0";
    }

    match CStr::from_bytes_until_nul(s.as_bytes()) {
        Ok(c) => Cow::from(c),
        // SAFETY: No internal 0 byte since already `FromBytesUntilNulError`
        Err(_) => unsafe { Cow::from(CString::new(s.as_bytes()).unwrap_unchecked()) },
    }
}

impl<'ctx> Builders<'ctx> {
    pub fn new(context: &'ctx Context, project: &ScratchProject) -> Self {
        let module = context.create_module("xyojit");
        let builder = context.create_builder();
        Self::link_generated_bitcodes(&module, context);
        let functions = Functions {
            llvm_floor: get_libm_f64_to_f64(&module, &context, "floor"),
            llvm_abs: get_libm_f64_to_f64(&module, &context, "fabs"),
            llvm_ceil: get_libm_f64_to_f64(&module, &context, "ceil"),
            llvm_sqrt: get_libm_f64_to_f64(&module, &context, "sqrt"),
            llvm_sin: get_libm_f64_to_f64(&module, &context, "sin"),
            llvm_cos: get_libm_f64_to_f64(&module, &context, "cos"),
            llvm_tan: get_libm_f64_to_f64(&module, &context, "tan"),
            llvm_asin: get_libm_f64_to_f64(&module, &context, "asin"),
            llvm_acos: get_libm_f64_to_f64(&module, &context, "acos"),
            llvm_atan: get_libm_f64_to_f64(&module, &context, "atan"),
            llvm_loge: get_libm_f64_to_f64(&module, &context, "log"),
            llvm_log10: get_libm_f64_to_f64(&module, &context, "log10"),
            llvm_exp: get_libm_f64_to_f64(&module, &context, "exp"),
            llvm_pow10: get_libm_f64_to_f64(&module, &context, "exp10"),
            str_to_num: module.get_function("xyo_atod").unwrap(),
            num_to_str: module.get_function("xyo_dtoa").unwrap(),
            str_cmp_gt: module.get_function("str_cmp_gt").unwrap(),
            str_cmp_lt: module.get_function("str_cmp_lt").unwrap(),
            str_cmp_eq: module.get_function("str_cmp_eq").unwrap(),
            is_num: module.get_function("str_is_num").unwrap(),
            str_to_bool: module.get_function("str_to_bool").unwrap(),
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
            let memory_buffer = unsafe {
                LLVMCreateMemoryBufferWithMemoryRange(
                    bytes.as_ptr() as *const libc::c_char,
                    bytes.len(),
                    to_c_str(name).as_ptr(),
                    false as i32,
                )
            };
            let buffer = unsafe { MemoryBuffer::new(memory_buffer) };
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

#[cfg(test)]
mod tests {
    use inkwell::context::Context;
    use serde_json::json;

    use super::*;

    fn project_with_variables() -> ScratchProject {
        serde_json::from_value(json!({
            "meta": {
                "semver": "3.0.0",
                "vm": null,
                "agent": null,
                "origin": null
            },
            "targets": [
                {
                    "isStage": true,
                    "name": "Stage",
                    "currentCostume": 0,
                    "blocks": {},
                    "variables": {
                        "global-score": ["score", 0],
                        "shared-id": ["shared", "stage"]
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
                },
                {
                    "isStage": false,
                    "name": "Sprite1",
                    "currentCostume": 0,
                    "blocks": {},
                    "variables": {
                        "local-health": ["health", 100],
                        "shared-id": ["shared local", "sprite"]
                    },
                    "lists": {},
                    "broadcasts": {},
                    "comments": null,
                    "costumes": [],
                    "sounds": [],
                    "visible": true,
                    "x": 0.0,
                    "y": 0.0,
                    "size": 100.0,
                    "direction": 90.0,
                    "draggable": false,
                    "rotationStyle": "all around",
                    "layerOrder": 1,
                    "volume": null
                }
            ]
        }))
        .unwrap()
    }

    #[test]
    fn builders_index_stage_variables_as_global_and_sprite_variables_as_local() {
        let context = Context::create();
        let project = project_with_variables();
        let builders = Builders::new(&context, &project);

        let global = builders.get_variable(1, "global-score").unwrap();
        assert!(global.is_global);
        assert!(global.variable_idx < 2);

        let local = builders.get_variable(1, "local-health").unwrap();
        assert!(!local.is_global);
        assert!(local.variable_idx < 2);

        assert!(builders.get_variable(0, "local-health").is_none());
    }

    #[test]
    fn get_variable_prefers_stage_global_when_ids_overlap() {
        let context = Context::create();
        let project = project_with_variables();
        let builders = Builders::new(&context, &project);

        let variable = builders.get_variable(1, "shared-id").unwrap();

        assert!(variable.is_global);
        assert!(variable.variable_idx < 2);
    }

    #[test]
    fn create_function_name_advances_in_stable_sequence() {
        let context = Context::create();
        let project = project_with_variables();
        let mut builders = Builders::new(&context, &project);

        let names = (0..28)
            .map(|_| builders.create_function_name())
            .collect::<Vec<_>>();

        assert_eq!(names[0], "func_a");
        assert_eq!(names[1], "func_b");
        assert_eq!(names[25], "func_z");
        assert_eq!(names[26], "func_ba");
        assert_eq!(names[27], "func_bb");
    }

    #[test]
    fn to_c_str_handles_empty_and_truncates_at_existing_nul() {
        assert_eq!(to_c_str("").to_bytes_with_nul(), b"\0");
        assert_eq!(to_c_str("abc\0ignored").to_bytes_with_nul(), b"abc\0");
    }
}
