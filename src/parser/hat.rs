use crate::{
    parser::{
        parser::parse_input,
        types::{GreaterTarget, HatStmt, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, Fields, ScratchProject, StageOrSprite},
};

pub fn is_hat_block(op: &BlockOpCodes) -> bool {
    match op {
        BlockOpCodes::EventWhenFlagClicked => true,
        BlockOpCodes::EventWhenKeyPressed => true,
        BlockOpCodes::EventWhenThisSpriteClicked => true,
        BlockOpCodes::EventWhenBackdropSwitchesTo => true,
        BlockOpCodes::EventWhenGreaterThan => true,
        BlockOpCodes::EventWhenBroadcastReceived => true,
        BlockOpCodes::ControlStartAsClone => true,
        BlockOpCodes::ProceduresDefinition => true,
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
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("KEY_OPTION").unwrap();
            let key_string = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            Ok(HatStmt::WhenKeyPressed {
                key: key_string.parse().unwrap(),
            })
        }
        BlockOpCodes::EventWhenThisSpriteClicked => Ok(HatStmt::WhenThisSpriteClicked),
        BlockOpCodes::EventWhenBackdropSwitchesTo => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("BACKDROP").unwrap();
            let backdrop = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            let stage = project
                .targets
                .iter()
                .find(|&v| matches!(v, StageOrSprite::Stage(_)));
            if let None = stage {
                return Err(ParserError::InvalidValue("Can't find stage"));
            }
            let stage = stage.unwrap();
            let costumes = match stage {
                StageOrSprite::Stage(v) => &v.costumes,
                StageOrSprite::Sprite(v) => &v.costumes,
            };
            let stage_idx: Option<usize> = {
                let mut found: Option<usize> = None;
                for i in 0..costumes.len() {
                    if costumes[i].name == *backdrop {
                        found = Some(i);
                        break;
                    }
                }
                found
            };
            if let None = stage_idx {
                return Err(ParserError::InvalidValue("Can't find costumes"));
            }
            Ok(HatStmt::WhenBacdropSwitchesTo {
                backdrop: stage_idx.unwrap(),
            })
        }
        BlockOpCodes::EventWhenBroadcastReceived => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("BROADCAST_OPTION").unwrap();
            let broadcast_id = match field {
                Fields::V2(v) => &v.1,
                _ => return Err(ParserError::InvalidValue("broadcast format")),
            };
            if let None = broadcast_id {
                return Err(ParserError::InvalidValue("broadcast format"));
            }
            Ok(HatStmt::WhenBroadcastReceived {
                target: broadcast_id.as_ref().unwrap().clone(),
            })
        }
        BlockOpCodes::EventWhenGreaterThan => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("WHENGREATERTHANMENU").unwrap();
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
            let inputs = block.inputs.as_ref().unwrap();
            let index_input = inputs.get("VALUE").unwrap();
            let idx = parse_input(project, target_idx, index_input).unwrap();
            Ok(HatStmt::WhenGreaterThan {
                target: listen_target,
                value: idx,
            })
        }
        BlockOpCodes::ControlStartAsClone => Ok(HatStmt::ControlStartAsClone),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
