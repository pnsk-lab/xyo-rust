use crate::{
    parser::{
        parser::parse_input,
        types::{
            CostumeStatueTarget, Expr, Literal, LooksEffects, LooksExpr, LooksFowardBackward,
            LooksFrontback, LooksStmt, ParseResult, ParserError,
        },
    },
    types::{Block, BlockOpCodes, Fields, Input, ScratchProject},
};
use std::collections::HashMap;

fn block_fields<'a>(
    block: &'a Block,
    missing_fields_error: &'static str,
) -> ParseResult<'a, &'a HashMap<String, Fields>> {
    block
        .fields
        .as_ref()
        .ok_or(ParserError::InvalidValue(missing_fields_error))
}

fn block_inputs<'a>(
    block: &'a Block,
    missing_inputs_error: &'static str,
) -> ParseResult<'a, &'a HashMap<String, Input>> {
    block
        .inputs
        .as_ref()
        .ok_or(ParserError::InvalidValue(missing_inputs_error))
}

fn required_field<'a>(
    fields: &'a HashMap<String, Fields>,
    key: &'static str,
    missing_field_error: &'static str,
) -> ParseResult<'a, &'a Fields> {
    fields
        .get(key)
        .ok_or(ParserError::InvalidValue(missing_field_error))
}

fn field_text(field: &Fields) -> &String {
    match field {
        Fields::V1(v) => &v.0,
        Fields::V2(v) => &v.0,
    }
}

fn required_expr_input<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    inputs: &'a HashMap<String, Input>,
    key: &'static str,
    missing_input_error: &'static str,
    parse_error: &'static str,
) -> ParseResult<'a, Expr> {
    let input = inputs
        .get(key)
        .ok_or(ParserError::InvalidValue(missing_input_error))?;
    parse_input(project, target_idx, input).map_err(|err| err.context(parse_error))
}

pub fn parse_looks_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::LooksCostumeNumberName => {
            let fields = block_fields(block, "missing fields in LooksCostumeNumberName block")?;
            let field = required_field(
                fields,
                "NUMBER_NAME",
                "missing NUMBER_NAME field in LooksCostumeNumberName block",
            )?;
            let number_or_name = field_text(field);
            let number_or_name = if number_or_name == "number" {
                CostumeStatueTarget::Number
            } else if number_or_name == "name" {
                CostumeStatueTarget::Name
            } else {
                return Err(ParserError::InvalidValue(
                    "NUMBER_NAME can supported in number or name",
                ));
            };
            Ok(Expr::Looks(LooksExpr::CostumeStatus {
                target: number_or_name,
            }))
        }
        BlockOpCodes::LooksBackdropNumberName => {
            let fields = block_fields(block, "missing fields in LooksBackdropNumberName block")?;
            let field = required_field(
                fields,
                "NUMBER_NAME",
                "missing NUMBER_NAME field in LooksBackdropNumberName block",
            )?;
            let number_or_name = field_text(field);
            let number_or_name = if number_or_name == "number" {
                CostumeStatueTarget::Number
            } else if number_or_name == "name" {
                CostumeStatueTarget::Name
            } else {
                return Err(ParserError::InvalidValue(
                    "NUMBER_NAME can supported in number or name",
                ));
            };
            Ok(Expr::Looks(LooksExpr::BackdropStatus {
                target: number_or_name,
            }))
        }
        BlockOpCodes::LooksSize => Ok(Expr::Looks(LooksExpr::Size)),
        BlockOpCodes::LooksCostume => {
            let fields = block_fields(block, "missing fields in LooksCostume block")?;
            let field = required_field(
                fields,
                "COSTUME",
                "missing COSTUME field in LooksCostume block",
            )?;
            let costume = field_text(field);
            Ok(Expr::Literal(Literal::String(costume.clone())))
        }
        BlockOpCodes::LooksBackdrops => {
            let fields = block_fields(block, "missing fields in LooksBackdrops block")?;
            let field = required_field(
                fields,
                "BACKDROP",
                "missing BACKDROP field in LooksBackdrops block",
            )?;
            let backdrop = field_text(field);
            Ok(Expr::Literal(Literal::String(backdrop.clone())))
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
pub fn parse_looks_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, LooksStmt> {
    match block.opcode {
        BlockOpCodes::LooksSayForSecs => {
            let inputs = block_inputs(block, "missing inputs in LooksSayForSecs block")?;
            let msg = required_expr_input(
                project,
                target_idx,
                inputs,
                "MESSAGE",
                "missing MESSAGE input",
                "failed to parse MESSAGE input in LooksSayForSecs block",
            )?;
            let secs = required_expr_input(
                project,
                target_idx,
                inputs,
                "SECS",
                "missing SECS input",
                "failed to parse SECS input in LooksSayForSecs block",
            )?;
            Ok(LooksStmt::SayForSecs { message: msg, secs })
        }
        BlockOpCodes::LooksSay => {
            let inputs = block_inputs(block, "missing inputs in LooksSay block")?;
            let msg = required_expr_input(
                project,
                target_idx,
                inputs,
                "MESSAGE",
                "missing MESSAGE input",
                "failed to parse MESSAGE input in LooksSay block",
            )?;
            Ok(LooksStmt::Say { message: msg })
        }
        BlockOpCodes::LooksThinkForSecs => {
            let inputs = block_inputs(block, "missing inputs in LooksThinkForSecs block")?;
            let msg = required_expr_input(
                project,
                target_idx,
                inputs,
                "MESSAGE",
                "missing MESSAGE input",
                "failed to parse MESSAGE input in LooksThinkForSecs block",
            )?;
            let secs = required_expr_input(
                project,
                target_idx,
                inputs,
                "SECS",
                "missing SECS input",
                "failed to parse SECS input in LooksThinkForSecs block",
            )?;
            Ok(LooksStmt::ThinkForSecs { message: msg, secs })
        }
        BlockOpCodes::LooksThink => {
            let inputs = block_inputs(block, "missing inputs in LooksThink block")?;
            let msg = required_expr_input(
                project,
                target_idx,
                inputs,
                "MESSAGE",
                "missing MESSAGE input",
                "failed to parse MESSAGE input in LooksThink block",
            )?;
            Ok(LooksStmt::Think { message: msg })
        }
        BlockOpCodes::LooksSwitchCostumeTo => {
            let inputs = block_inputs(block, "missing inputs in LooksSwitchCostumeTo block")?;
            let costume = required_expr_input(
                project,
                target_idx,
                inputs,
                "COSTUME",
                "missing COSTUME input",
                "failed to parse COSTUME input in LooksSwitchCostumeTo block",
            )?;
            Ok(LooksStmt::SwitchCostumeTo { costume })
        }
        BlockOpCodes::LooksNextCostume => Ok(LooksStmt::NextCostume),
        BlockOpCodes::LooksSwitchBackdropTo => {
            let inputs = block_inputs(block, "missing inputs in LooksSwitchBackdropTo block")?;
            let backdrop = required_expr_input(
                project,
                target_idx,
                inputs,
                "BACKDROP",
                "missing BACKDROP input",
                "failed to parse BACKDROP input in LooksSwitchBackdropTo block",
            )?;
            Ok(LooksStmt::SwitchBackdropTo { backdrop })
        }
        BlockOpCodes::LooksNextBackdrop => Ok(LooksStmt::NextBackdrop),
        BlockOpCodes::LooksChangeSizeBy => {
            let inputs = block_inputs(block, "missing inputs in LooksChangeSizeBy block")?;
            let change = required_expr_input(
                project,
                target_idx,
                inputs,
                "CHANGE",
                "missing CHANGE input",
                "failed to parse CHANGE input in LooksChangeSizeBy block",
            )?;
            Ok(LooksStmt::ChangeSizeBy { change })
        }
        BlockOpCodes::LooksSetSizeTo => {
            let inputs = block_inputs(block, "missing inputs in LooksSetSizeTo block")?;
            let size = required_expr_input(
                project,
                target_idx,
                inputs,
                "SIZE",
                "missing SIZE input",
                "failed to parse SIZE input in LooksSetSizeTo block",
            )?;
            Ok(LooksStmt::SetSizeTo { size })
        }
        BlockOpCodes::LooksChangeEffectBy => {
            let inputs = block_inputs(block, "missing inputs in LooksChangeEffectBy block")?;
            let change = required_expr_input(
                project,
                target_idx,
                inputs,
                "CHANGE",
                "missing CHANGE input",
                "failed to parse CHANGE input in LooksChangeEffectBy block",
            )?;
            let fields = block_fields(block, "missing fields in LooksChangeEffectBy block")?;
            let field = required_field(
                fields,
                "EFFECT",
                "missing EFFECT field in LooksChangeEffectBy block",
            )?;
            let effect = field_text(field);
            let effect = match effect.as_str() {
                "COLOR" => LooksEffects::Color,
                "FISHEYE" => LooksEffects::Fisheye,
                "WHIRL" => LooksEffects::Whirl,
                "PIXELATE" => LooksEffects::Pixelate,
                "MOSAIC" => LooksEffects::Mosaic,
                "BRIGHTNESS" => LooksEffects::Brightness,
                "GHOST" => LooksEffects::Ghost,
                _ => return Err(ParserError::InvalidValue("Unknown effect")),
            };
            Ok(LooksStmt::ChangeEffectBy { change, effect })
        }
        BlockOpCodes::LooksSetEffectTo => {
            let inputs = block_inputs(block, "missing inputs in LooksSetEffectTo block")?;
            let value = required_expr_input(
                project,
                target_idx,
                inputs,
                "VALUE",
                "missing VALUE input",
                "failed to parse VALUE input in LooksSetEffectTo block",
            )?;
            let fields = block_fields(block, "missing fields in LooksSetEffectTo block")?;
            let field = required_field(
                fields,
                "EFFECT",
                "missing EFFECT field in LooksSetEffectTo block",
            )?;
            let effect = field_text(field);
            let effect = match effect.as_str() {
                "COLOR" => LooksEffects::Color,
                "FISHEYE" => LooksEffects::Fisheye,
                "WHIRL" => LooksEffects::Whirl,
                "PIXELATE" => LooksEffects::Pixelate,
                "MOSAIC" => LooksEffects::Mosaic,
                "BRIGHTNESS" => LooksEffects::Brightness,
                "GHOST" => LooksEffects::Ghost,
                _ => return Err(ParserError::InvalidValue("Unknown effect")),
            };
            Ok(LooksStmt::SetEffectTo { value, effect })
        }
        BlockOpCodes::LooksClearGraphicEffects => Ok(LooksStmt::ClearEffects),
        BlockOpCodes::LooksShow => Ok(LooksStmt::Show),
        BlockOpCodes::LooksHide => Ok(LooksStmt::Hide),
        BlockOpCodes::LooksGotoFrontBack => {
            let fields = block_fields(block, "missing fields in LooksGotoFrontBack block")?;
            let field = required_field(
                fields,
                "FRONT_BACK",
                "missing FRONT_BACK field in LooksGotoFrontBack block",
            )?;
            let frontback = field_text(field);
            let frontback = match frontback.as_str() {
                "front" => LooksFrontback::Front,
                "back" => LooksFrontback::Back,
                _ => return Err(ParserError::InvalidValue("expected front or back")),
            };
            Ok(LooksStmt::GotoFrontback {
                frontback: frontback,
            })
        }
        BlockOpCodes::LooksGoForwardBackwardLayers => {
            let fields = block_fields(
                block,
                "missing fields in LooksGoForwardBackwardLayers block",
            )?;
            let field = required_field(
                fields,
                "FORWARD_BACKWARD",
                "missing FORWARD_BACKWARD field in LooksGoForwardBackwardLayers block",
            )?;
            let forward_backward = field_text(field);
            let forward_backward = match forward_backward.as_str() {
                "forward" => LooksFowardBackward::Forward,
                "backward" => LooksFowardBackward::Backward,
                _ => return Err(ParserError::InvalidValue("expected forward or backward")),
            };
            Ok(LooksStmt::GotoForwardBackwardLayers { forward_backward })
        }
        BlockOpCodes::LooksChangeStretchBy => {
            let inputs = block_inputs(block, "missing inputs in LooksChangeStretchBy block")?;
            let change = required_expr_input(
                project,
                target_idx,
                inputs,
                "CHANGE",
                "missing CHANGE input",
                "failed to parse CHANGE input in LooksChangeStretchBy block",
            )?;
            Ok(LooksStmt::ChangeStretchBy { change })
        }
        BlockOpCodes::LooksSetStretchTo => {
            let inputs = block_inputs(block, "missing inputs in LooksSetStretchTo block")?;
            let stretch = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRETCH",
                "missing STRETCH input",
                "failed to parse STRETCH input in LooksSetStretchTo block",
            )?;
            Ok(LooksStmt::SetStretchTo { stretch })
        }
        BlockOpCodes::LooksHideAllSprites => Ok(LooksStmt::HideAllSprites),
        BlockOpCodes::LooksSwitchBackdropToAndWait => {
            let inputs = block_inputs(
                block,
                "missing inputs in LooksSwitchBackdropToAndWait block",
            )?;
            let backdrop = required_expr_input(
                project,
                target_idx,
                inputs,
                "BACKDROP",
                "missing BACKDROP input",
                "failed to parse BACKDROP input in LooksSwitchBackdropToAndWait block",
            )?;
            Ok(LooksStmt::SwitchBackdropToAndWait { backdrop })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
