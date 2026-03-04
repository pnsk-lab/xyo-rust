use inkwell::values::FunctionValue;

use crate::{
    compiler::{
        compiler::generate_expr_ir,
        types::{Builders, SpriteKeys, create_sprite_struct_type},
    },
    parser::types::MotionStmt,
};

pub fn parse_motion_stmt<'ctx>(
    builders: &Builders<'ctx>,
    stmt: &MotionStmt,
    function: &FunctionValue<'ctx>,
    strings: &mut Vec<String>,
) {
    match stmt {
        MotionStmt::SetX { x } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteX.into(), "field0")
                .unwrap();
            let val = generate_expr_ir(builders, x, function, strings);
            builders.builder.build_store(field_ptr, val).unwrap();
        }
        MotionStmt::ChangeXBy { dx } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteX.into(), "field0")
                .unwrap();
            let val = generate_expr_ir(builders, dx, function, strings);
            let old_val = builders
                .builder
                .build_load(builders.context.f64_type(), field_ptr, "old_x")
                .unwrap()
                .into_float_value();
            let new_val = builders
                .builder
                .build_float_add(old_val, val, "new_x")
                .unwrap();
            builders.builder.build_store(field_ptr, new_val).unwrap();
        }
        MotionStmt::SetY { y } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteY.into(), "field0")
                .unwrap();
            let val = generate_expr_ir(builders, y, function, strings);
            builders.builder.build_store(field_ptr, val).unwrap();
        }
        MotionStmt::ChangeYBy { dy } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteY.into(), "field0")
                .unwrap();
            let val = generate_expr_ir(builders, dy, function, strings);
            let old_val = builders
                .builder
                .build_load(builders.context.f64_type(), field_ptr, "old_y")
                .unwrap()
                .into_float_value();
            let new_val = builders
                .builder
                .build_float_add(old_val, val, "new_y")
                .unwrap();
            builders.builder.build_store(field_ptr, new_val).unwrap();
        }
        _ => todo!("やります"),
    }
}
