use inkwell::values::FunctionValue;

use crate::{
    compiler::{compiler::ScratchReturnTypes, types::Builders},
    parser::types::Literal,
};

pub fn parse_literal_expr<'ctx>(
    builders: &Builders<'ctx>,
    expr: &Literal,
    _function: &FunctionValue<'ctx>,
    strings: &mut Vec<String>,
) -> ScratchReturnTypes<'ctx> {
    match expr {
        Literal::Number(v) => {
            strings.push(v.clone());
            ScratchReturnTypes::StringLiteral((
                v.clone(),
                builders
                    .context
                    .i64_type()
                    .const_int(strings.len() as u64 - 1, false),
            ))
        }
        Literal::String(v) => {
            strings.push(v.clone());
            ScratchReturnTypes::StringLiteral((
                v.clone(),
                builders
                    .context
                    .i64_type()
                    .const_int(strings.len() as u64 - 1, false),
            ))
        }
        _ => todo!("あとでやる"),
    }
}
