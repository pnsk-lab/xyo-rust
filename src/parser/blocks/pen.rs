use crate::{
    parser::types::{ParseResult, PenExpr},
    types::{Block, ScratchProject},
};

pub fn parse_pen_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, PenExpr> {
    Err(crate::parser::types::ParserError::NotHandledOp(
        block.opcode,
    ))
}
