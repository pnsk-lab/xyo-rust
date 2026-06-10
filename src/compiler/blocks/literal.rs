use inkwell::{AddressSpace, values::FunctionValue};

use crate::{
    compiler::{
        compiler::ScratchReturnTypes,
        types::{Builders, create_dynamic_struct_type},
        utils::create_string_struct,
    },
    parser::types::Literal,
};

pub fn parse_literal_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &Literal,
    _function: &FunctionValue<'ctx>,
    target_idx: usize,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        Literal::Number(v) => {
            ScratchReturnTypes::StringLiteral((v.clone(), create_string_struct(builders, v)))
        }
        Literal::String(v) => {
            ScratchReturnTypes::StringLiteral((v.clone(), create_string_struct(builders, v)))
        }
        Literal::Variable { target } => {
            let variable = builders.get_variable(target_idx, target);
            if variable.is_none() {
                panic!("存在しない変数を参照してるな、ゆるさん")
            }
            let variable = variable.unwrap();
            let global_ref = builders.get_global_variable_ptr(variable);
            let global_ref_ref = builders
                .builder
                .build_load(
                    builders.context.ptr_type(AddressSpace::default()),
                    global_ref,
                    "f",
                )
                .unwrap()
                .into_pointer_value();
            ScratchReturnTypes::Dynamic(global_ref_ref)
        }
        _ => todo!("あとでやる"),
    }
}
