use crate::{
    parser::types::{ParseResult, ProceduresExpr},
    types::{Block, ScratchProject},
};

pub fn parse_procedures_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, ProceduresExpr> {
    Err(crate::parser::types::ParserError::NotHandledOp(
        block.opcode,
    ))
}
