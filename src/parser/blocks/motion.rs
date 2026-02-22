use crate::{
    parser::types::{MotionExpr, ParseResult, ParserError},
    types::{Block, BlockOpCodes, ScratchProject},
};

pub fn parse_motion_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, MotionExpr> {
    match block.opcode {
        BlockOpCodes::MotionXPosition => Ok(MotionExpr::XPosition),
        BlockOpCodes::MotionYPosition => Ok(MotionExpr::YPosition),
        BlockOpCodes::MotionDirection => Ok(MotionExpr::Direction),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
