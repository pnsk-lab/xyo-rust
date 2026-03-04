use crate::{
    parser::{
        parser::parse_input,
        types::{Expr, GreaterTarget, HatStmt, ParseResult, ParserError, ProceduresExpr},
    },
    types::{Block, BlockOpCodes, Fields, ScratchProject, StageOrSprite},
};

pub fn is_hat_block(op: &BlockOpCodes) -> bool {
    match op {
        BlockOpCodes::EventWhenFlagClicked => true,
        BlockOpCodes::EventWhenKeyPressed => true,
        BlockOpCodes::EventWhenThisSpriteClicked => true,
        BlockOpCodes::EventWhenStageClicked => true,
        BlockOpCodes::EventWhenBackdropSwitchesTo => true,
        BlockOpCodes::EventWhenGreaterThan => true,
        BlockOpCodes::EventWhenBroadcastReceived => true,
        BlockOpCodes::ControlStartAsClone => true,
        BlockOpCodes::ProceduresDefinition => true,
        BlockOpCodes::EventWhenTouchingObject => true,
        _ => false,
    }
}
pub fn parse_hat<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, HatStmt> {
    match block.opcode {
        BlockOpCodes::EventWhenFlagClicked => Ok(HatStmt::WhenFlagClicked),
        BlockOpCodes::EventWhenKeyPressed => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in EventWhenKeyPressed block",
            ))?;
            let field = fields
                .get("KEY_OPTION")
                .ok_or(ParserError::InvalidValue("missing KEY_OPTION field"))?;
            let key_string = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            Ok(HatStmt::WhenKeyPressed {
                key: key_string.clone(),
            })
        }
        BlockOpCodes::EventWhenThisSpriteClicked => Ok(HatStmt::WhenThisSpriteClicked),
        BlockOpCodes::EventWhenStageClicked => Ok(HatStmt::WhenStageClicked),
        BlockOpCodes::EventWhenBackdropSwitchesTo => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in EventWhenBackdropSwitchesTo block",
            ))?;
            let field = fields
                .get("BACKDROP")
                .ok_or(ParserError::InvalidValue("missing BACKDROP field"))?;
            let backdrop = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            let stage = project
                .targets
                .iter()
                .find(|&v| matches!(v, StageOrSprite::Stage(_)))
                .ok_or(ParserError::InvalidValue("Can't find stage"))?;
            let costumes = match stage {
                StageOrSprite::Stage(v) => &v.costumes,
                StageOrSprite::Sprite(v) => &v.costumes,
            };
            let stage_idx = costumes
                .iter()
                .position(|costume| costume.name == *backdrop)
                .ok_or(ParserError::InvalidValue("Can't find costumes"))?;
            Ok(HatStmt::WhenBacdropSwitchesTo {
                backdrop: stage_idx,
            })
        }
        BlockOpCodes::EventWhenBroadcastReceived => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in EventWhenBroadcastReceived block",
            ))?;
            let field = fields
                .get("BROADCAST_OPTION")
                .ok_or(ParserError::InvalidValue("missing BROADCAST_OPTION field"))?;
            let broadcast_id = match field {
                Fields::V2(v) => &v.1,
                _ => return Err(ParserError::InvalidValue("broadcast format")),
            };
            let broadcast_id = broadcast_id
                .as_ref()
                .ok_or(ParserError::InvalidValue("broadcast format"))?;
            Ok(HatStmt::WhenBroadcastReceived {
                target: broadcast_id.clone(),
            })
        }
        BlockOpCodes::EventWhenGreaterThan => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in EventWhenGreaterThan block",
            ))?;
            let field = fields
                .get("WHENGREATERTHANMENU")
                .ok_or(ParserError::InvalidValue(
                    "missing WHENGREATERTHANMENU field",
                ))?;
            let listen_target = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            let listen_target = if listen_target == "LOUDNESS" {
                GreaterTarget::Volume
            } else if listen_target == "TIMER" {
                GreaterTarget::Timer
            } else {
                return Err(ParserError::InvalidValue(
                    "WHENGREATERTHANMENU is only supported TIMER or LOUDNESS",
                ));
            };
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in EventWhenGreaterThan block",
            ))?;
            let index_input = inputs
                .get("VALUE")
                .ok_or(ParserError::InvalidValue("missing VALUE input"))?;
            let idx = parse_input(project, target_idx, index_input).map_err(|err| {
                err.context("failed to parse VALUE input in EventWhenGreaterThan block")
            })?;
            Ok(HatStmt::WhenGreaterThan {
                target: listen_target,
                value: idx,
            })
        }
        BlockOpCodes::ControlStartAsClone => Ok(HatStmt::ControlStartAsClone),
        BlockOpCodes::ProceduresDefinition => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in ProceduresDefinition block",
            ))?;
            let index_input = inputs
                .get("custom_block")
                .ok_or(ParserError::InvalidValue("missing custom_block input"))?;
            let prototype = parse_input(project, target_idx, index_input).map_err(|err| {
                err.context("failed to parse custom_block input in ProceduresDefinition block")
            })?;
            if let Expr::Procedures(p) = prototype {
                match p {
                    ProceduresExpr::ProceduresPrototype { prototype } => {
                        Ok(HatStmt::ProcedureDefinition { prototype })
                    }
                    _ => Err(ParserError::InvalidValue(
                        "procedures custom_block must be ProceduresPrototype",
                    )),
                }
            } else {
                Err(ParserError::InvalidValue(
                    "procedures custom_block must be ProceduresPrototype",
                ))
            }
        }
        BlockOpCodes::EventWhenTouchingObject => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in EventWhenTouchingObject block",
            ))?;
            let index_input = inputs
                .get("TOUCHINGOBJECTMENU")
                .ok_or(ParserError::InvalidValue(
                    "missing TOUCHINGOBJECTMENU input",
                ))?;
            let target = parse_input(project, target_idx, index_input).map_err(|err| {
                err.context(
                    "failed to parse TOUCHINGOBJECTMENU input in EventWhenTouchingObject block",
                )
            })?;
            Ok(HatStmt::WhenTouchingObject { object: target })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
