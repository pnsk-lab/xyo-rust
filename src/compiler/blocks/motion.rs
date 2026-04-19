use inkwell::values::FunctionValue;

use crate::{
    compiler::{
        compiler::generate_expr_ir,
        types::{Builders, SpriteKeys, create_sprite_struct_type},
        utils::scratch_return_to_number,
    },
    parser::types::MotionStmt,
};

pub fn parse_motion_stmt<'ctx>(
    builders: &Builders<'ctx>,
    stmt: &MotionStmt,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
) {
    match stmt {
        MotionStmt::SetX { x } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteX.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, x, function, target_idx),
                function,
            );
            builders.builder.build_store(field_ptr, val).unwrap();
        }
        MotionStmt::ChangeXBy { dx } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteX.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, dx, function, target_idx),
                function,
            );
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
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, y, function, target_idx),
                function,
            );
            builders.builder.build_store(field_ptr, val).unwrap();
        }
        MotionStmt::ChangeYBy { dy } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteY.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, dy, function, target_idx),
                function,
            );
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
        MotionStmt::GotoXY { x, y } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr_x = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteX.into(), "field0")
                .unwrap();
            let field_ptr_y = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteY.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, x, function, target_idx),
                function,
            );
            builders.builder.build_store(field_ptr_x, val).unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, y, function, target_idx),
                function,
            );
            builders.builder.build_store(field_ptr_y, val).unwrap();
        }
        MotionStmt::TurnRight { degrees } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteRotate.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, degrees, function, target_idx),
                function,
            );
            let old_val = builders
                .builder
                .build_load(builders.context.f64_type(), field_ptr, "old_degree")
                .unwrap()
                .into_float_value();
            let new_val = builders
                .builder
                .build_float_add(old_val, val, "old_degree")
                .unwrap();
            builders.builder.build_store(field_ptr, new_val).unwrap();
        }
        MotionStmt::TurnLeft { degrees } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteRotate.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, degrees, function, target_idx),
                function,
            );
            let old_val = builders
                .builder
                .build_load(builders.context.f64_type(), field_ptr, "old_degree")
                .unwrap()
                .into_float_value();
            let new_val = builders
                .builder
                .build_float_sub(old_val, val, "old_degree")
                .unwrap();
            builders.builder.build_store(field_ptr, new_val).unwrap();
        }
        MotionStmt::PointInDirection { direction } => {
            let p = function.get_first_param().unwrap().into_pointer_value();
            let sprite_type = create_sprite_struct_type(builders.context);
            let field_ptr = builders
                .builder
                .build_struct_gep(sprite_type, p, SpriteKeys::SpriteRotate.into(), "field0")
                .unwrap();
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, direction, function, target_idx),
                function,
            );
            builders.builder.build_store(field_ptr, val).unwrap();
        }
        _ => todo!("やります"),
    }
}
