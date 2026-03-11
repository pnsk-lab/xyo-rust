use std::collections::HashMap;

use inkwell::{
    AddressSpace, attributes::AttributeLoc, builder::Builder, context::Context, module::Module,
    types::StructType, values::FunctionValue,
};

use crate::compiler::utils::build_xor_shift_128_plus;

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
    counter: usize,
    pub functions: Functions<'ctx>,
}
pub struct Functions<'ctx> {
    pub llvm_floor: FunctionValue<'ctx>,
    pub str_to_num: FunctionValue<'ctx>,
    pub num_to_str: FunctionValue<'ctx>,
    pub bool_to_str: FunctionValue<'ctx>,
    pub str_cmp_gt: FunctionValue<'ctx>,
    pub is_num: FunctionValue<'ctx>,
    pub rand: FunctionValue<'ctx>,
}

impl<'ctx> Builders<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
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
        let functions = Functions {
            llvm_floor: module.add_function("llvm.floor.f64", llvm_floor, None),
            str_to_num: module.add_function("xyo_str_to_num", str_to_num_func_type, None),
            num_to_str: module.add_function("xyo_num_to_str", num_to_str_func_type, None),
            bool_to_str: module.add_function("xyo_bool_to_str", bool_to_str, None),
            str_cmp_gt: module.add_function("xyo_str_cmp_gt", str_cmp_gt, None),
            is_num: module.add_function("str_is_num", str_is_num_func_type, None),
            rand: build_xor_shift_128_plus(&context, &module),
        };
        Self {
            context,
            module,
            builder,
            counter: 0,
            functions,
        }
    }
    pub fn create_function_name(&mut self) -> String {
        let mut n = self.counter;
        if n == 0 {
            self.counter += 1;
            return "a".to_string();
        }

        let mut chars = Vec::new();

        while n > 0 {
            let idx = (n % 26) as u8;
            chars.push((b'a' + idx) as char);
            n /= 26;
        }

        self.counter += 1;

        chars.iter().rev().collect()
    }
}
