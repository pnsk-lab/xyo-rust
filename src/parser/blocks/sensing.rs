use crate::{
    parser::{
        parser::parse_input,
        types::{Expr, Literal, ParseResult, ParserError, SensingExpr, SensingStmt, TimeTarget},
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

pub fn parse_sensing_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::SensingTouchingObject => {
            let inputs = block_inputs(block, "missing inputs in SensingTouchingObject block")?;
            let object = required_expr_input(
                project,
                target_idx,
                inputs,
                "TOUCHINGOBJECTMENU",
                "missing TOUCHINGOBJECTMENU input",
                "failed to parse TOUCHINGOBJECTMENU input in SensingTouchingObject block",
            )?;
            Ok(Expr::Sensing(SensingExpr::TouchingObject {
                target: Box::new(object),
            }))
        }
        BlockOpCodes::SensingTouchingObjectMenu => {
            let fields = block_fields(block, "missing fields in SensingTouchingObjectMenu block")?;
            let field = required_field(
                fields,
                "TOUCHINGOBJECTMENU",
                "missing TOUCHINGOBJECTMENU field in SensingTouchingObjectMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingTouchingColor => {
            let inputs = block_inputs(block, "missing inputs in SensingTouchingColor block")?;
            let color = required_expr_input(
                project,
                target_idx,
                inputs,
                "COLOR",
                "missing COLOR input",
                "failed to parse COLOR input in SensingTouchingColor block",
            )?;
            Ok(Expr::Sensing(SensingExpr::TouchingColor {
                target: Box::new(color),
            }))
        }
        BlockOpCodes::SensingColorIsTouchingColor => {
            let inputs =
                block_inputs(block, "missing inputs in SensingColorIsTouchingColor block")?;
            let color = required_expr_input(
                project,
                target_idx,
                inputs,
                "COLOR",
                "missing COLOR input",
                "failed to parse COLOR input in SensingColorIsTouchingColor block",
            )?;
            let color2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "COLOR2",
                "missing COLOR2 input",
                "failed to parse COLOR2 input in SensingColorIsTouchingColor block",
            )?;
            Ok(Expr::Sensing(SensingExpr::ColorTouchingColor {
                target: Box::new(color),
                base: Box::new(color2),
            }))
        }
        BlockOpCodes::SensingDistanceTo => {
            let inputs = block_inputs(block, "missing inputs in SensingDistanceTo block")?;
            let object = required_expr_input(
                project,
                target_idx,
                inputs,
                "DISTANCETOMENU",
                "missing DISTANCETOMENU input",
                "failed to parse DISTANCETOMENU input in SensingDistanceTo block",
            )?;
            Ok(Expr::Sensing(SensingExpr::DistanceBy {
                target: Box::new(object),
            }))
        }
        BlockOpCodes::SensingDistanceToMenu => {
            let fields = block_fields(block, "missing fields in SensingDistanceToMenu block")?;
            let field = required_field(
                fields,
                "DISTANCETOMENU",
                "missing DISTANCETOMENU field in SensingDistanceToMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingAnswer => Ok(Expr::Sensing(SensingExpr::Answer)),
        BlockOpCodes::SensingKeyPressed => {
            let inputs = block_inputs(block, "missing inputs in SensingKeyPressed block")?;
            let object = required_expr_input(
                project,
                target_idx,
                inputs,
                "KEY_OPTION",
                "missing KEY_OPTION input",
                "failed to parse KEY_OPTION input in SensingKeyPressed block",
            )?;
            Ok(Expr::Sensing(SensingExpr::IsKeyDown {
                target: Box::new(object),
            }))
        }
        BlockOpCodes::SensingKeyOptions => {
            let fields = block_fields(block, "missing fields in SensingKeyOptions block")?;
            let field = required_field(
                fields,
                "KEY_OPTION",
                "missing KEY_OPTION field in SensingKeyOptions block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingMouseDown => Ok(Expr::Sensing(SensingExpr::IsMouseDown)),
        BlockOpCodes::SensingMouseX => Ok(Expr::Sensing(SensingExpr::MouseX)),
        BlockOpCodes::SensingMouseY => Ok(Expr::Sensing(SensingExpr::MouseY)),
        BlockOpCodes::SensingLoudness => Ok(Expr::Sensing(SensingExpr::Volume)),
        BlockOpCodes::SensingTimer => Ok(Expr::Sensing(SensingExpr::Timer)),
        BlockOpCodes::SensingOf => {
            let inputs = block_inputs(block, "missing inputs in SensingOf block")?;
            let object = required_expr_input(
                project,
                target_idx,
                inputs,
                "OBJECT",
                "missing OBJECT input",
                "failed to parse OBJECT input in SensingOf block",
            )?;
            let fields = block_fields(block, "missing fields in SensingOf block")?;
            let field = required_field(
                fields,
                "PROPERTY",
                "missing PROPERTY field in SensingOf block",
            )?;
            let operator = field_text(field);
            let operator = match operator.as_str() {
                "backdrop #" => crate::parser::types::StatusTarget::CostumeNumber,
                "backdrop name" => crate::parser::types::StatusTarget::CostumeName,
                "costume #" => crate::parser::types::StatusTarget::CostumeNumber,
                "costume name" => crate::parser::types::StatusTarget::CostumeName,
                "x position" => crate::parser::types::StatusTarget::XPosition,
                "y position" => crate::parser::types::StatusTarget::YPosition,
                "direction" => crate::parser::types::StatusTarget::Direction,
                "volume" => crate::parser::types::StatusTarget::Volume,
                "size" => crate::parser::types::StatusTarget::Size,
                v => crate::parser::types::StatusTarget::Variable(v.to_string()),
            };
            Ok(Expr::Sensing(SensingExpr::SpriteStatus {
                target: Box::new(object),
                item: operator,
            }))
        }
        BlockOpCodes::SensingOfObjectMenu => {
            let fields = block_fields(block, "missing fields in SensingOfObjectMenu block")?;
            let field = required_field(
                fields,
                "OBJECT",
                "missing OBJECT field in SensingOfObjectMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingUsername => Ok(Expr::Sensing(SensingExpr::Username)),
        BlockOpCodes::SensingUserid => Ok(Expr::Sensing(SensingExpr::Userid)),
        BlockOpCodes::SensingOnline => Ok(Expr::Sensing(SensingExpr::Online)),
        BlockOpCodes::SensingDaysSince2000 => Ok(Expr::Sensing(SensingExpr::Since2000Days)),
        BlockOpCodes::SensingCurrent => {
            let fields = block_fields(block, "missing fields in SensingCurrent block")?;
            let field = required_field(
                fields,
                "CURRENTMENU",
                "missing CURRENTMENU field in SensingCurrent block",
            )?;
            let time_target = field_text(field);
            let time_target = match time_target.as_str() {
                "YEAR" => TimeTarget::Year,
                "MONTH" => TimeTarget::Month,
                "DATE" => TimeTarget::Day,
                "DAYOFWEEK" => TimeTarget::Date,
                "HOUR" => TimeTarget::Hour,
                "MINUTE" => TimeTarget::Minute,
                "SECOND" => TimeTarget::Second,
                _ => return Err(ParserError::InvalidValue("unknown time target")),
            };
            Ok(Expr::Sensing(SensingExpr::NowTime { time: time_target }))
        }
        BlockOpCodes::SensingLoud => Ok(Expr::Sensing(SensingExpr::IsLoud)),
        _ => Err(crate::parser::types::ParserError::NotHandledOp(
            block.opcode,
        )),
    }
}

pub fn parse_sensing_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, SensingStmt> {
    match block.opcode {
        BlockOpCodes::SensingAskAndWait => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in SensingAskAndWait block",
            ))?;
            let question = inputs
                .get("QUESTION")
                .ok_or(ParserError::InvalidValue("missing QUESTION input"))?;
            let question = parse_input(project, target_idx, question).map_err(|err| {
                err.context("failed to parse QUESTION in SensingAskAndWait block")
            })?;
            Ok(SensingStmt::AskAndWait { question })
        }
        BlockOpCodes::SensingSetDragMode => {
            let fields = block_fields(block, "missing fields in SensingSetDragMode block")?;
            let field = required_field(
                fields,
                "DRAG_MODE",
                "missing DRAG_MODE field in SensingSetDragMode block",
            )?;
            let draggable = field_text(field);
            let draggable = match draggable.as_str() {
                "draggable" => true,
                "not draggable" => false,
                _ => return Err(ParserError::InvalidValue("unknown draggable status")),
            };
            Ok(SensingStmt::SetDraggable { draggable })
        }
        BlockOpCodes::SensingResetTimer => Ok(SensingStmt::ResetTimer),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
