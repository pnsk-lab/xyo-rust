use inkwell::values::FunctionValue;

use crate::{
    compiler::{compiler::ScratchReturnTypes, types::Builders, utils::create_string_struct},
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
        _ => todo!("あとでやる"),
    }
}
