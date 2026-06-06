use inkwell::values::FunctionValue;

use crate::{
    compiler::{
        compiler::ScratchReturnTypes,
        types::{Builders, SpriteKeys, create_sprite_struct_type},
    },
    parser::types::LooksExpr,
};

pub fn parse_looks_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &LooksExpr,
    function: &FunctionValue<'ctx>,
    _: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        LooksExpr::Size => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let size_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteSize.into(), "field2")
                .unwrap();
            let size_val = builders
                .builder
                .build_load(builders.context.f64_type(), size_ptr, "size")
                .unwrap()
                .into_float_value();
            ScratchReturnTypes::Number(size_val)
        }
        _ => todo!("未実装"),
    }
}
