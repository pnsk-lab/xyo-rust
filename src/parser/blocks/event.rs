use crate::{
    parser::types::{EventExpr, ParseResult},
    types::{Block, ScratchProject},
};

pub fn parse_event_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, EventExpr> {
    Err(crate::parser::types::ParserError::NotHandledOp(
        block.opcode,
    ))
}
