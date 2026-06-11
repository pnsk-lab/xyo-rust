use crate::{
    parser::{
        parser::parse_input,
        types::{EventExpr, EventStmt, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, ScratchProject},
};

pub fn parse_event_expr<'a>(_: &'a ScratchProject, _: usize, block: &'a Block) -> ParseResult<'a, EventExpr> {
    Err(crate::parser::types::ParserError::NotHandledOp(block.opcode))
}

pub fn parse_event_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, EventStmt> {
    match block.opcode {
        BlockOpCodes::EventBroadcast => {
            let inputs = block
                .inputs
                .as_ref()
                .ok_or(ParserError::InvalidValue("missing inputs in EventBroadcast block"))?;
            let broadcast = inputs
                .get("BROADCAST_INPUT")
                .ok_or(ParserError::InvalidValue("missing BROADCAST_INPUT input"))?;
            let broadcast = parse_input(project, target_idx, broadcast)
                .map_err(|err| err.context("failed to parse BROADCAST_INPUT in EventBroadcast block"))?;
            Ok(EventStmt::Broadcast { target: broadcast })
        }
        BlockOpCodes::EventBroadcastAndWait => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in EventBroadcastAndWait block",
            ))?;
            let broadcast = inputs
                .get("BROADCAST_INPUT")
                .ok_or(ParserError::InvalidValue("missing BROADCAST_INPUT input"))?;
            let broadcast = parse_input(project, target_idx, broadcast)
                .map_err(|err| err.context("failed to parse BROADCAST_INPUT in EventBroadcastAndWait block"))?;
            Ok(EventStmt::BroadcastAndWait { target: broadcast })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
