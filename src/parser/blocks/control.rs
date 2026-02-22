use crate::{
    parser::types::{ControlExpr, ParseResult},
    types::{Block, ScratchProject},
};

pub fn parse_control_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, ControlExpr> {
    Err(crate::parser::types::ParserError::NotHandledOp(
        block.opcode,
    ))
}
