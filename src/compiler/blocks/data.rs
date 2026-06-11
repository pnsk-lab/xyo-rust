use inkwell::values::FunctionValue;

use crate::{
    compiler::{
        compiler::generate_expr_ir,
        types::{Builders, CompilerState},
        utils::scratch_return_to_dynamic,
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
            );
            builders.builder.build_store(variable_global, dynamic_value).unwrap();
        }
        _ => todo!("やる気がない"),
    }
}
