use std::collections::HashMap;

use crate::{
    parser::{
        parser::parse_input,
        types::{Expr, Literal, ParseResult, ParserError, PenStmt},
    },
    types::{Block, BlockOpCodes, Fields, Input, ScratchProject},
};
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

pub fn parse_pen_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::PenMenuColorParam => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in PenMenuColorParam block",
            ))?;
            let color_param = required_field(
                fields,
                "colorParam",
                "missing colorParam field in PenMenuColorParam block",
            )?;
            let color_param = field_text(color_param);
            Ok(Expr::Literal(Literal::String(color_param.clone())))
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}

pub fn parse_pen_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, PenStmt> {
    match block.opcode {
        BlockOpCodes::PenClear => Ok(PenStmt::PenClear),
        BlockOpCodes::PenStamp => Ok(PenStmt::PenStamp),
        BlockOpCodes::PenDown => Ok(PenStmt::PenDown),
        BlockOpCodes::PenUp => Ok(PenStmt::PenUp),
        BlockOpCodes::PenSetPenColorToColor => {
            let inputs = block_inputs(block, "missing inputs in PenSetPenColorToColor block")?;
            let color = required_expr_input(
                project,
                target_idx,
                inputs,
                "COLOR",
                "missing COLOR input",
                "failed to parse COLOR input in PenSetPenColorToColor block",
            )?;
            Ok(PenStmt::SetPenColorToColor { color })
        }
        BlockOpCodes::PenChangePenColorParamBy => {
            let inputs = block_inputs(block, "missing inputs in PenChangePenColorParamBy block")?;
            let color_param = required_expr_input(
                project,
                target_idx,
                inputs,
                "COLOR_PARAM",
                "missing COLOR_PARAM input",
                "failed to parse COLOR_PARAM input in PenChangePenColorParamBy block",
            )?;
            let value = required_expr_input(
                project,
                target_idx,
                inputs,
                "VALUE",
                "missing VALUE input",
                "failed to parse VALUE input in PenChangePenColorParamBy block",
            )?;
            Ok(PenStmt::ChangePenColorParamBy { color_param, value })
        }
        BlockOpCodes::PenSetPenColorParamTo => {
            let inputs = block_inputs(block, "missing inputs in PenSetPenColorParamTo block")?;
            let color_param = required_expr_input(
                project,
                target_idx,
                inputs,
                "COLOR_PARAM",
                "missing COLOR_PARAM input",
                "failed to parse COLOR_PARAM input in PenSetPenColorParamTo block",
            )?;
            let value = required_expr_input(
                project,
                target_idx,
                inputs,
                "VALUE",
                "missing VALUE input",
                "failed to parse VALUE input in PenSetPenColorParamTo block",
            )?;
            Ok(PenStmt::SetPenColorParamTo { color_param, value })
        }
        BlockOpCodes::PenChangePenSizeBy => {
            let inputs = block_inputs(block, "missing inputs in PenChangePenSizeBy block")?;
            let size = required_expr_input(
                project,
                target_idx,
                inputs,
                "SIZE",
                "missing SIZE input",
                "failed to parse SIZE input in PenChangePenSizeBy block",
            )?;
            Ok(PenStmt::ChangePenSizeBy { size })
        }
        BlockOpCodes::PenSetPenSizeTo => {
            let inputs = block_inputs(block, "missing inputs in PenSetPenSizeTo block")?;
            let size = required_expr_input(
                project,
                target_idx,
                inputs,
                "SIZE",
                "missing SIZE input",
                "failed to parse SIZE input in PenSetPenSizeTo block",
            )?;
            Ok(PenStmt::SetPenSizeTo { size })
        }
        BlockOpCodes::PenChangePenHueBy => {
            let inputs = block_inputs(block, "missing inputs in PenChangePenHueBy block")?;
            let hue = required_expr_input(
                project,
                target_idx,
                inputs,
                "HUE",
                "missing HUE input",
                "failed to parse HUE input in PenChangePenHueBy block",
            )?;
            Ok(PenStmt::ChangePenHueBy { hue })
        }
        BlockOpCodes::PenSetPenHueToNumber => {
            let inputs = block_inputs(block, "missing inputs in PenSetPenHueToNumber block")?;
            let hue = required_expr_input(
                project,
                target_idx,
                inputs,
                "HUE",
                "missing HUE input",
                "failed to parse HUE input in PenSetPenHueToNumber block",
            )?;
            Ok(PenStmt::SetPenHueTo { hue })
        }
        BlockOpCodes::PenChangePenShadeBy => {
            let inputs = block_inputs(block, "missing inputs in PenChangePenShadeBy block")?;
            let shade = required_expr_input(
                project,
                target_idx,
                inputs,
                "SHADE",
                "missing SHADE input",
                "failed to parse SHADE input in PenChangePenShadeBy block",
            )?;
            Ok(PenStmt::ChangePenShadeBy { shade })
        }
        BlockOpCodes::PenSetPenShadeToNumber => {
            let inputs = block_inputs(block, "missing inputs in PenSetPenShadeToNumber block")?;
            let shade = required_expr_input(
                project,
                target_idx,
                inputs,
                "SHADE",
                "missing SHADE input",
                "failed to parse SHADE input in PenSetPenShadeToNumber block",
            )?;
            Ok(PenStmt::SetPenShadeTo { shade })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
