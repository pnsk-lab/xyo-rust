use crate::{
    parser::types::{ParseResult, ParserError, SoundExpr},
    types::{Block, BlockOpCodes, ScratchProject},
};

pub fn parse_sound_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, SoundExpr> {
    match block.opcode {
        BlockOpCodes::SoundVolume => Ok(SoundExpr::Volume),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
