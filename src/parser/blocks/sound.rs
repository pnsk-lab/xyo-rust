use crate::{
    parser::{
        parser::parse_input,
        types::{Expr, Literal, ParseResult, ParserError, SoundEffect, SoundExpr, SoundStmt},
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

pub fn parse_sound_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::SoundVolume => Ok(Expr::Sound(SoundExpr::Volume)),
        BlockOpCodes::SoundSoundsMenu => {
            let fields = block_fields(block, "missing fields in SoundSoundsMenu block")?;
            let field = required_field(
                fields,
                "SOUND_MENU",
                "missing SOUND_MENU field in SoundSoundsMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SoundBeatsMenu => {
            let fields = block_fields(block, "missing fields in SoundBeatsMenu block")?;
            let field = required_field(
                fields,
                "BEATS",
                "missing BEATS field in SoundBeatsMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SoundEffectsMenu => {
            let fields = block_fields(block, "missing fields in SoundEffectsMenu block")?;
            let field = required_field(
                fields,
                "EFFECT",
                "missing EFFECT field in SoundEffectsMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}

pub fn parse_sound_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, SoundStmt> {
    match block.opcode {
        BlockOpCodes::SoundPlayUntilDone => {
            let inputs = block_inputs(block, "missing inputs in SoundPlayUntilDone block")?;
            let sound_menu = required_expr_input(
                project,
                target_idx,
                inputs,
                "SOUND_MENU",
                "missing SOUND_MENU input",
                "failed to parse SOUND_MENU input in SoundPlayUntilDone block",
            )?;
            Ok(SoundStmt::PlayUntilDone { sound: sound_menu })
        }
        BlockOpCodes::SoundPlay => {
            let inputs = block_inputs(block, "missing inputs in SoundPlay block")?;
            let sound_menu = required_expr_input(
                project,
                target_idx,
                inputs,
                "SOUND_MENU",
                "missing SOUND_MENU input",
                "failed to parse SOUND_MENU input in SoundPlay block",
            )?;
            Ok(SoundStmt::Play { sound: sound_menu })
        }
        BlockOpCodes::SoundStopAllSounds => Ok(SoundStmt::StopAllSounds),
        BlockOpCodes::SoundChangeEffectBy => {
            let inputs = block_inputs(block, "missing inputs in SoundChangeEffectBy block")?;
            let value = required_expr_input(
                project,
                target_idx,
                inputs,
                "VALUE",
                "missing VALUE input",
                "failed to parse VALUE input in SoundChangeEffectBy block",
            )?;
            let fields = block_fields(block, "missing fields in SoundChangeEffectBy block")?;
            let field = required_field(
                fields,
                "EFFECT",
                "missing EFFECT field in SoundChangeEffectBy block",
            )?;
            let effect = field_text(field);
            let effect = match effect.as_str() {
                "PITCH" => SoundEffect::Pitch,
                "PAN" => SoundEffect::Pan,
                _ => return Err(ParserError::InvalidValue("Unknown SoundEffect")),
            };
            Ok(SoundStmt::ChangeSoundEffectBy {
                value,
                target: effect,
            })
        }
        BlockOpCodes::SoundSetEffectTo => {
            let inputs = block_inputs(block, "missing inputs in SoundSetEffectTo block")?;
            let value = required_expr_input(
                project,
                target_idx,
                inputs,
                "VALUE",
                "missing VALUE input",
                "failed to parse VALUE input in SoundSetEffectTo block",
            )?;
            let fields = block_fields(block, "missing fields in SoundSetEffectTo block")?;
            let field = required_field(
                fields,
                "EFFECT",
                "missing EFFECT field in SoundSetEffectTo block",
            )?;
            let effect = field_text(field);
            let effect = match effect.as_str() {
                "PITCH" => SoundEffect::Pitch,
                "PAN" => SoundEffect::Pan,
                _ => return Err(ParserError::InvalidValue("Unknown SoundEffect")),
            };
            Ok(SoundStmt::SetSoundEffectTo {
                value,
                target: effect,
            })
        }
        BlockOpCodes::SoundClearEffects => Ok(SoundStmt::ClearSoundEffect),
        BlockOpCodes::SoundChangeVolumeBy => {
            let inputs = block_inputs(block, "missing inputs in SoundChangeVolumeBy block")?;
            let volume = required_expr_input(
                project,
                target_idx,
                inputs,
                "VOLUME",
                "missing VOLUME input",
                "failed to parse VOLUME input in SoundChangeVolumeBy block",
            )?;
            Ok(SoundStmt::ChangeVolumeBy { value: volume })
        }
        BlockOpCodes::SoundSetVolumeTo => {
            let inputs = block_inputs(block, "missing inputs in SoundSetVolumeTo block")?;
            let volume = required_expr_input(
                project,
                target_idx,
                inputs,
                "VOLUME",
                "missing VOLUME input",
                "failed to parse VOLUME input in SoundSetVolumeTo block",
            )?;
            Ok(SoundStmt::SetVolumeTo { value: volume })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
