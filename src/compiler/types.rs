use std::collections::HashMap;

use inkwell::{
    AddressSpace, attributes::AttributeLoc, builder::Builder, context::Context, module::Module,
    types::StructType, values::FunctionValue,
};

use crate::{
    compiler::utils::build_xor_shift_128_plus,
    types::{ScratchProject, StageOrSprite},
};

#[repr(C)]
pub struct SpriteStruct {
    pub string_map: &'static HashMap<u64, String>,
    pub sprite_x: f64,
    pub sprite_y: f64,
    pub sprite_rotate: f64,
}
pub fn create_sprite_struct_type<'a>(context: &'a Context) -> StructType<'a> {
    context.struct_type(
        &[
            context.ptr_type(AddressSpace::default()).into(),
            context.f64_type().into(),
            context.f64_type().into(),
            context.f64_type().into(),
        ],
        false,
    )
}

pub enum SpriteKeys {
    StringMap,
    SpriteX,
    SpriteY,
    SpriteRotate,
}

impl From<SpriteKeys> for u32 {
    fn from(field: SpriteKeys) -> Self {
        match field {
            SpriteKeys::StringMap => 0,
            SpriteKeys::SpriteX => 1,
            SpriteKeys::SpriteY => 2,
            SpriteKeys::SpriteRotate => 3,
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
}
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
    pub bool_to_str: FunctionValue<'ctx>,
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

impl<'ctx> Builders<'ctx> {
    pub fn new(context: &'ctx Context, project: &ScratchProject) -> Self {
        let module = context.create_module("xyojit");
        let builder = context.create_builder();
        let ptr_type = context.ptr_type(AddressSpace::default());
        let f64_type = context.f64_type();
        let i64_type = context.i64_type();
        let i1_type = context.bool_type();
        let llvm_floor = f64_type.fn_type(&[f64_type.into()], false);
        let str_to_num_func_type = f64_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let str_is_num_func_type = i1_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let num_to_str_func_type = i64_type.fn_type(&[ptr_type.into(), f64_type.into()], false);
        let bool_to_str = i64_type.fn_type(&[ptr_type.into(), i1_type.into()], false);
        let str_cmp_gt =
            i1_type.fn_type(&[ptr_type.into(), i64_type.into(), i64_type.into()], false);
        let str_cmp_lt =
            i1_type.fn_type(&[ptr_type.into(), i64_type.into(), i64_type.into()], false);
        let str_cmp_eq =
            i1_type.fn_type(&[ptr_type.into(), i64_type.into(), i64_type.into()], false);
        let str_to_bool = i1_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let functions = Functions {
            llvm_floor: module.add_function("llvm.floor.f64", llvm_floor, None),
            llvm_abs: module.add_function("llvm.fabs.f64", llvm_floor, None),
            llvm_ceil: module.add_function("llvm.ceil.f64", llvm_floor, None),
            llvm_sqrt: module.add_function("llvm.sqrt.f64", llvm_floor, None),
            llvm_sin: module.add_function("llvm.sin.f64", llvm_floor, None),
            llvm_cos: module.add_function("llvm.cos.f64", llvm_floor, None),
            llvm_tan: module.add_function("llvm.tan.f64", llvm_floor, None),
            llvm_asin: module.add_function("llvm.asin.f64", llvm_floor, None),
            llvm_acos: module.add_function("llvm.acos.f64", llvm_floor, None),
            llvm_atan: module.add_function("llvm.atan.f64", llvm_floor, None),
            llvm_loge: module.add_function("llvm.log.f64", llvm_floor, None),
            llvm_log10: module.add_function("llvm.log10.f64", llvm_floor, None),
            llvm_exp: module.add_function("llvm.exp.f64", llvm_floor, None),
            llvm_pow10: module.add_function("llvm.exp10.f64", llvm_floor, None),
            str_to_num: module.add_function("xyo_str_to_num", str_to_num_func_type, None),
            num_to_str: module.add_function("xyo_num_to_str", num_to_str_func_type, None),
            bool_to_str: module.add_function("xyo_bool_to_str", bool_to_str, None),
            str_cmp_gt: module.add_function("xyo_str_cmp_gt", str_cmp_gt, None),
            str_cmp_lt: module.add_function("xyo_str_cmp_lt", str_cmp_lt, None),
            str_cmp_eq: module.add_function("xyo_str_cmp_eq", str_cmp_eq, None),
            is_num: module.add_function("str_is_num", str_is_num_func_type, None),
            str_to_bool: module.add_function("str_to_bool", str_to_bool, None),
            rand: build_xor_shift_128_plus(&context, &module),
        };
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
