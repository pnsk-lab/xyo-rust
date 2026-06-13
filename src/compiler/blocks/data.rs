use inkwell::values::FunctionValue;

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
            let scratch_return_value = ScratchReturnTypes::Dynamic(variable_global);
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
