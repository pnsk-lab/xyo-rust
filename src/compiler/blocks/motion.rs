use inkwell::{
    values::{FloatValue, FunctionValue, PointerValue},
    FloatPredicate,
};

use crate::{
    compiler::{
        compiler::generate_expr_ir,
        types::{
            create_costume_struct_type, create_sprite_struct_type, Builders, CostumeInfoKeys,
            SpriteKeys,
        },
        utils::scratch_return_to_number,
    },
    parser::types::MotionStmt,
};

pub fn get_x_ptr<'ctx>(
    builders: &Builders<'ctx>,
    function: &FunctionValue<'ctx>,
) -> PointerValue<'ctx> {
    let p = function.get_first_param().unwrap().into_pointer_value();
    let sprite_type = create_sprite_struct_type(builders.context);
    builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteX.into(), "field0")
        .unwrap()
}
pub fn get_y_ptr<'ctx>(
    builders: &Builders<'ctx>,
    function: &FunctionValue<'ctx>,
) -> PointerValue<'ctx> {
    let p = function.get_first_param().unwrap().into_pointer_value();
    let sprite_type = create_sprite_struct_type(builders.context);
    builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteY.into(), "field1")
        .unwrap()
}
pub fn fence_goto<'ctx>(
    builders: &Builders<'ctx>,
    function: &FunctionValue<'ctx>,
    x: Option<&FloatValue>,
    y: Option<&FloatValue>,
) {
    let p = function.get_first_param().unwrap().into_pointer_value();
    let sprite_type = create_sprite_struct_type(builders.context);
    let f64_type = builders.context.f64_type();
    let fence_width = f64_type.const_float(15.0);

    let size_ptr = builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteSize.into(), "field2")
        .unwrap();
    let size_val = builders
        .builder
        .build_load(f64_type, size_ptr, "size")
        .unwrap()
        .into_float_value();
    let costume_id_ptr = builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteCostumeId.into(), "field2")
        .unwrap();
    let costume_id = builders
        .builder
        .build_load(builders.context.i64_type(), costume_id_ptr, "costume")
        .unwrap()
        .into_int_value();
    let costumes_pointer = builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteCostumes.into(), "sprites")
        .unwrap();
    let costume_type = create_costume_struct_type(builders.context);
    let costumes_base_ptr = builders
        .builder
        .build_load(
            builders.context.ptr_type(inkwell::AddressSpace::default()),
            costumes_pointer,
            "sprite_costumes",
        )
        .unwrap()
        .into_pointer_value();
    let costumes_missing = builders
        .builder
        .build_is_null(costumes_base_ptr, "sprite_costumes_is_null")
        .unwrap();

    let with_costume_block = builders.context.append_basic_block(*function, "fence_with_costume");
    let without_costume_block =
        builders.context.append_basic_block(*function, "fence_without_costume");
    let merge_block = builders.context.append_basic_block(*function, "fence_merge");
    builders
        .builder
        .build_conditional_branch(costumes_missing, without_costume_block, with_costume_block)
        .unwrap();

    builders.builder.position_at_end(with_costume_block);
    let costume_ptr = unsafe {
        builders
            .builder
            .build_in_bounds_gep(
                costume_type,
                costumes_base_ptr,
                &[costume_id],
                "sprite_costume_n_ptr",
            )
            .unwrap()
    };

    let width_ptr = builders
        .builder
        .build_struct_gep(
            costume_type,
            costume_ptr,
            CostumeInfoKeys::Width.into(),
            "width_ptr",
        )
        .unwrap();
    let width_val = builders
        .builder
        .build_load(f64_type, width_ptr, "width")
        .unwrap()
        .into_float_value();
    let height_ptr = builders
        .builder
        .build_struct_gep(
            costume_type,
            costume_ptr,
            CostumeInfoKeys::Height.into(),
            "height_ptr",
        )
        .unwrap();
    let height_val = builders
        .builder
        .build_load(f64_type, height_ptr, "height")
        .unwrap()
        .into_float_value();

    let min_width_height = builders
        .builder
        .build_select(
            builders
                .builder
                .build_float_compare(FloatPredicate::OGT, width_val, height_val, "comp")
                .unwrap(),
            height_val,
            width_val,
            "min",
        )
        .unwrap()
        .into_float_value();

    let scaled_min_axis = builders
        .builder
        .build_float_mul(
            min_width_height,
            builders
                .builder
                .build_float_div(size_val, f64_type.const_float(100.0), "div")
                .unwrap(),
            "mul",
        )
        .unwrap();

    let half_axis = builders
        .builder
        .build_float_div(scaled_min_axis, f64_type.const_float(2.0), "half_axis")
        .unwrap();
    let inset_with_costume = builders
        .builder
        .build_call(
            builders.functions.llvm_floor,
            &[half_axis.into()],
            "fence_inset_floor",
        )
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_float_value();
    let inset_with_costume = builders
        .builder
        .build_select(
            builders
                .builder
                .build_float_compare(
                    FloatPredicate::OLT,
                    fence_width,
                    inset_with_costume,
                    "fence_width_lt_inset",
                )
                .unwrap(),
            fence_width,
            inset_with_costume,
            "fence_inset",
        )
        .unwrap()
        .into_float_value();
    builders
        .builder
        .build_unconditional_branch(merge_block)
        .unwrap();

    builders.builder.position_at_end(without_costume_block);
    builders
        .builder
        .build_unconditional_branch(merge_block)
        .unwrap();

    builders.builder.position_at_end(merge_block);
    let inset = builders
        .builder
        .build_phi(f64_type, "fence_inset_phi")
        .unwrap();
    inset.add_incoming(&[
        (&inset_with_costume, with_costume_block),
        (&fence_width, without_costume_block),
    ]);
    let inset = inset.as_basic_value().into_float_value();

    let x = match x {
        Some(v) => *v,
        None => builders
            .builder
            .build_load(f64_type, get_x_ptr(builders, function), "current_x")
            .unwrap()
            .into_float_value(),
    };
    let y = match y {
        Some(v) => *v,
        None => builders
            .builder
            .build_load(f64_type, get_y_ptr(builders, function), "current_y")
            .unwrap()
            .into_float_value(),
    };

    let stage_half_width = f64_type.const_float(240.0);
    let stage_half_height = f64_type.const_float(180.0);
    let limit_x = builders
        .builder
        .build_float_sub(stage_half_width, inset, "limit_x")
        .unwrap();
    let limit_y = builders
        .builder
        .build_float_sub(stage_half_height, inset, "limit_y")
        .unwrap();

    let x = builders
        .builder
        .build_select(
            builders
                .builder
                .build_float_compare(FloatPredicate::OLT, x, limit_x, "x_lt_limit")
                .unwrap(),
            builders
                .builder
                .build_select(
                    builders
                        .builder
                        .build_float_compare(
                            FloatPredicate::OGT,
                            x,
                            builders
                                .builder
                                .build_float_mul(
                                    limit_x,
                                    builders.context.f64_type().const_float(-1.0),
                                    "neg",
                                )
                                .unwrap(),
                            "x_gt_neg_limit",
                        )
                        .unwrap(),
                    x,
                    builders
                        .builder
                        .build_float_mul(
                            limit_x,
                            builders.context.f64_type().const_float(-1.0),
                            "neg",
                        )
                        .unwrap(),
                    "x_mid",
                )
                .unwrap()
                .into_float_value(),
            limit_x,
            "x_fenced",
        )
        .unwrap()
        .into_float_value();

    let y = builders
        .builder
        .build_select(
            builders
                .builder
                .build_float_compare(FloatPredicate::OLT, y, limit_y, "y_lt_limit")
                .unwrap(),
            builders
                .builder
                .build_select(
                    builders
                        .builder
                        .build_float_compare(
                            FloatPredicate::OGT,
                            y,
                            builders
                                .builder
                                .build_float_mul(
                                    limit_y,
                                    builders.context.f64_type().const_float(-1.0),
                                    "neg",
                                )
                                .unwrap(),
                            "y_gt_neg_limit",
                        )
                        .unwrap(),
                    y,
                    builders
                        .builder
                        .build_float_mul(
                            limit_y,
                            builders.context.f64_type().const_float(-1.0),
                            "neg",
                        )
                        .unwrap(),
                    "y_mid",
                )
                .unwrap()
                .into_float_value(),
            limit_y,
            "y_fenced",
        )
        .unwrap()
        .into_float_value();

    let x_ptr = get_x_ptr(builders, function);
    builders.builder.build_store(x_ptr, x).unwrap();
    let y_ptr = get_y_ptr(builders, function);
    builders.builder.build_store(y_ptr, y).unwrap();
}

pub fn parse_motion_stmt<'ctx>(
    builders: &Builders<'ctx>,
    stmt: &MotionStmt,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
) {
    match stmt {
        MotionStmt::SetX { x } => {
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, x, function, target_idx),
                function,
            );
            fence_goto(builders, function, Some(&val), None);
        }
        MotionStmt::ChangeXBy { dx } => {
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, dx, function, target_idx),
                function,
            );
            let field_ptr = get_x_ptr(builders, function);
            let old_val = builders
                .builder
                .build_load(builders.context.f64_type(), field_ptr, "old_x")
                .unwrap()
                .into_float_value();
            let new_val = builders
                .builder
                .build_float_add(old_val, val, "new_x")
                .unwrap();
            fence_goto(builders, function, Some(&new_val), None);
        }
        MotionStmt::SetY { y } => {
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, y, function, target_idx),
                function,
            );
            fence_goto(builders, function, None, Some(&val));
        }
        MotionStmt::ChangeYBy { dy } => {
            let val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, dy, function, target_idx),
                function,
            );
            let field_ptr = get_y_ptr(builders, function);
            let old_val = builders
                .builder
                .build_load(builders.context.f64_type(), field_ptr, "old_y")
                .unwrap()
                .into_float_value();
            let new_val = builders
                .builder
                .build_float_add(old_val, val, "new_y")
                .unwrap();
            fence_goto(builders, function, None, Some(&new_val));
        }
        MotionStmt::GotoXY { x, y } => {
            let x_val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, x, function, target_idx),
                function,
            );
            let y_val = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, y, function, target_idx),
                function,
            );
            fence_goto(builders, function, Some(&x_val), Some(&y_val));
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
