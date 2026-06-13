use inkwell::values::{FloatValue, FunctionValue, PointerValue};

use crate::{
    compiler::{
        compiler::{ScratchReturnTypes, generate_expr_ir},
        types::{
            Builders, CompilerState, CostumeInfoKeys, SpriteKeys, create_costume_struct_type, create_sprite_struct_type,
        },
        utils::{scratch_return_to_number, scratch_return_to_string},
    },
    parser::types::{LooksExpr, LooksStmt},
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

fn get_size_ptr<'ctx>(builders: &Builders<'ctx>, function: &FunctionValue<'ctx>) -> PointerValue<'ctx> {
    let p = function.get_first_param().unwrap().into_pointer_value();
    let sprite_type = create_sprite_struct_type(builders.context);
    builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteSize.into(), "field2")
        .unwrap()
}
fn set_size_to<'ctx>(builders: &Builders<'ctx>, size: FloatValue<'ctx>, function: &FunctionValue) {
    let p = function.get_first_param().unwrap().into_pointer_value();
    let sprite_type = create_sprite_struct_type(builders.context);
    let f64_type = builders.context.f64_type();

    let size_ptr = builders
        .builder
        .build_struct_gep(sprite_type, p, SpriteKeys::SpriteSize.into(), "field2")
        .unwrap();
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
    let costume_ptr = unsafe {
        builders
            .builder
            .build_in_bounds_gep(costume_type, costumes_base_ptr, &[costume_id], "sprite_costume_n_ptr")
            .unwrap()
    };

    let width_ptr = builders
        .builder
        .build_struct_gep(costume_type, costume_ptr, CostumeInfoKeys::Width.into(), "width_ptr")
        .unwrap();
    let width_val = builders
        .builder
        .build_load(f64_type, width_ptr, "width")
        .unwrap()
        .into_float_value();
    let height_ptr = builders
        .builder
        .build_struct_gep(costume_type, costume_ptr, CostumeInfoKeys::Height.into(), "height_ptr")
        .unwrap();
    let height_val = builders
        .builder
        .build_load(f64_type, height_ptr, "height")
        .unwrap()
        .into_float_value();

    let min_scale = builders
        .builder
        .build_call(
            builders.functions.llvm_min,
            &[
                builders.context.f64_type().const_float(1.0).into(),
                builders
                    .builder
                    .build_call(
                        builders.functions.llvm_max,
                        &[
                            builders
                                .builder
                                .build_float_div(builders.context.f64_type().const_float(5.0), width_val, "scale_width")
                                .unwrap()
                                .into(),
                            builders
                                .builder
                                .build_float_div(
                                    builders.context.f64_type().const_float(5.0),
                                    height_val,
                                    "scale_height",
                                )
                                .unwrap()
                                .into(),
                        ],
                        "scale",
                    )
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
                    .unwrap()
                    .into_float_value()
                    .into(),
            ],
            "name",
        )
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_float_value();
    let max_scale = builders
        .builder
        .build_call(
            builders.functions.llvm_min,
            &[
                builders
                    .builder
                    .build_float_div(
                        builders.context.f64_type().const_float(1.5 * 480.0).into(),
                        width_val,
                        "max_scale_width",
                    )
                    .unwrap()
                    .into(),
                builders
                    .builder
                    .build_float_div(
                        builders.context.f64_type().const_float(1.5 * 480.0).into(),
                        height_val,
                        "max_scale_height",
                    )
                    .unwrap()
                    .into(),
            ],
            "max_scale",
        )
        .unwrap()
        .try_as_basic_value()
        .basic()
        .unwrap()
        .into_float_value();
    let new_size = builders
        .builder
        .build_float_mul(
            builders
                .builder
                .build_call(
                    builders.functions.llvm_max,
                    &[
                        builders
                            .builder
                            .build_call(
                                builders.functions.llvm_min,
                                &[
                                    builders
                                        .builder
                                        .build_float_div(
                                            size,
                                            builders.context.f64_type().const_float(100.0),
                                            "size_div_100",
                                        )
                                        .unwrap()
                                        .into(),
                                    max_scale.into(),
                                ],
                                "clamp_max",
                            )
                            .unwrap()
                            .try_as_basic_value()
                            .basic()
                            .unwrap()
                            .into_float_value()
                            .into(),
                        min_scale.into(),
                    ],
                    "clamp_min",
                )
                .unwrap()
                .try_as_basic_value()
                .basic()
                .unwrap()
                .into_float_value(),
            builders.context.f64_type().const_float(100.0),
            "mul_100",
        )
        .unwrap();
    builders.builder.build_store(size_ptr, new_size).unwrap();
}

pub fn parse_looks_stmt<'ctx>(
    builders: &Builders<'ctx>,
    stmt: &LooksStmt,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
    compiler_state: &mut CompilerState,
) {
    match stmt {
        LooksStmt::SetSizeTo { size } => {
            let size = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, size, function, target_idx),
                function,
            );
            set_size_to(builders, size, function);
            compiler_state.request_redraw = true;
        }
        LooksStmt::ChangeSizeBy { change } => {
            let change = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, change, function, target_idx),
                function,
            );
            let old_size_ptr = get_size_ptr(builders, function);
            let old_size = builders
                .builder
                .build_load(builders.context.f64_type(), old_size_ptr, "size")
                .unwrap()
                .into_float_value();
            let new_size = builders.builder.build_float_add(old_size, change, "new_size").unwrap();
            set_size_to(builders, new_size, function);
            compiler_state.request_redraw = true;
        }
        LooksStmt::Say { message } => {
            let s = scratch_return_to_string(
                builders,
                &generate_expr_ir(builders, message, function, target_idx),
                function,
            );
            builders
                .builder
                .build_call(builders.functions.print, &[s.into()], "say")
                .unwrap();
        }
        _ => todo!("未実装!!!"),
    }
}
