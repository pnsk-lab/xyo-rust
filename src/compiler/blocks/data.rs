use inkwell::{AddressSpace, values::FunctionValue};

use crate::{
    compiler::{
        compiler::{ScratchReturnTypes, generate_expr_ir},
        types::{Builders, CompilerState},
        utils::{scratch_return_to_dynamic, scratch_return_to_number},
    },
    parser::types::DataStmt,
};

pub fn parse_data_stmt<'ctx>(
    builders: &Builders<'ctx>,
    stmt: &DataStmt,
    function: &FunctionValue<'ctx>,
    target_idx: usize,
    _compiler_state: &mut CompilerState,
) {
    match stmt {
        DataStmt::SetVariable { value, variable } => {
            let variable_global =
                builders.get_global_variable_ptr(builders.get_variable(target_idx, variable).unwrap());
            let dynamic_value = scratch_return_to_dynamic(
                builders,
                &generate_expr_ir(builders, value, function, target_idx),
                function,
                Some(variable_global),
            );
            builders.builder.build_store(variable_global, dynamic_value).unwrap();
        }
        DataStmt::ChangeVariableBy { value, variable } => {
            let variable_global =
                builders.get_global_variable_ptr(builders.get_variable(target_idx, variable).unwrap());
            let diff_value = scratch_return_to_number(
                builders,
                &generate_expr_ir(builders, value, function, target_idx),
                function,
            );
            let old_dynamic = builders
                .builder
                .build_load(
                    builders.context.ptr_type(AddressSpace::default()),
                    variable_global,
                    "old_dynamic",
                )
                .unwrap()
                .into_pointer_value();
            let scratch_return_value = ScratchReturnTypes::Dynamic(old_dynamic);
            let old_value = scratch_return_to_number(builders, &scratch_return_value, function);
            builders
                .builder
                .build_store(
                    variable_global,
                    scratch_return_to_dynamic(
                        builders,
                        &ScratchReturnTypes::Number(
                            builders.builder.build_float_add(old_value, diff_value, "add").unwrap(),
                        ),
                        function,
                        Some(variable_global),
                    ),
                )
                .unwrap();
        }
        _ => todo!("やる気がない"),
    }
}

#[cfg(test)]
mod tests {
    use inkwell::{AddressSpace, context::Context, values::FunctionValue};
    use serde_json::json;

    use super::*;
    use crate::{
        compiler::types::Builders,
        parser::types::{Expr, Literal},
        types::ScratchProject,
    };

    fn project_with_global_variable() -> ScratchProject {
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
                        "score-id": ["score", 41]
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
                    "variables": {},
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

    fn test_function<'ctx>(context: &'ctx Context, builders: &Builders<'ctx>) -> FunctionValue<'ctx> {
        let ptr_type = context.ptr_type(AddressSpace::default());
        let fn_type = context.void_type().fn_type(&[ptr_type.into()], false);
        let function = builders.module.add_function("test", fn_type, None);
        let entry = context.append_basic_block(function, "entry");
        builders.builder.position_at_end(entry);
        function
    }

    #[test]
    fn change_variable_by_loads_global_slot_before_reading_dynamic() {
        let context = Context::create();
        let project = project_with_global_variable();
        let builders = Builders::new(&context, &project);
        let function = test_function(&context, &builders);
        let mut compiler_state = CompilerState {
            request_redraw: false,
            has_terminator: false,
        };

        parse_data_stmt(
            &builders,
            &DataStmt::ChangeVariableBy {
                value: Expr::Literal(Literal::Number("1".to_string())),
                variable: "score-id".to_string(),
            },
            &function,
            1,
            &mut compiler_state,
        );
        builders.builder.build_return(None).unwrap();

        assert!(builders.module.verify().is_ok(), "{}", builders.module.to_string());
        let ir = builders.module.to_string();
        assert!(ir.contains("%old_dynamic = load ptr, ptr @global_0"), "{ir}");
        assert!(!ir.contains("load i8, ptr @global_0"), "{ir}");
        assert!(
            !ir.contains("getelementptr inbounds nuw ({ i8, ptr }, ptr @global_0"),
            "{ir}"
        );
    }
}
