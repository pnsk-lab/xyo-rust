use crate::{
    parser::{
        parser::parse_input,
        types::{Expr, Literal, MotionExpr, MotionStmt, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, Fields, Input, RotationStyle, ScratchProject},
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

pub fn parse_motion_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::MotionXPosition => Ok(Expr::Motion(MotionExpr::XPosition)),
        BlockOpCodes::MotionYPosition => Ok(Expr::Motion(MotionExpr::YPosition)),
        BlockOpCodes::MotionDirection => Ok(Expr::Motion(MotionExpr::Direction)),
        BlockOpCodes::MotionGoToMenu => {
            let fields = block_fields(block, "missing fields in MotionGoToMenu block")?;
            let field = required_field(fields, "TO", "missing TO field in MotionGoToMenu block")?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::MotionGlideToMenu => {
            let fields = block_fields(block, "missing fields in MotionGlideToMenu block")?;
            let field =
                required_field(fields, "TO", "missing TO field in MotionGlideToMenu block")?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::MotionPointTowardsMenu => {
            let fields = block_fields(block, "missing fields in MotionPointTowardsMenu block")?;
            let field = required_field(
                fields,
                "TOWARDS",
                "missing TOWARDS field in MotionPointTowardsMenu block",
            )?;
            let operator = field_text(field);
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
pub fn parse_motion_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, MotionStmt> {
    match block.opcode {
        BlockOpCodes::MotionMoveSteps => {
            let inputs = block_inputs(block, "missing inputs in MotionMoveSteps block")?;
            let step = required_expr_input(
                project,
                target_idx,
                inputs,
                "STEPS",
                "missing STEPS input",
                "failed to parse STEPS input in MotionMoveSteps block",
            )?;
            Ok(MotionStmt::MoveStep { steps: step })
        }
        BlockOpCodes::MotionTurnRight => {
            let inputs = block_inputs(block, "missing inputs in MotionTurnRight block")?;
            let degrees = required_expr_input(
                project,
                target_idx,
                inputs,
                "DEGREES",
                "missing DEGREES input",
                "failed to parse DEGREES input in MotionTurnRight block",
            )?;
            Ok(MotionStmt::TurnRight { degrees })
        }
        BlockOpCodes::MotionTurnLeft => {
            let inputs = block_inputs(block, "missing inputs in MotionTurnLeft block")?;
            let degrees = required_expr_input(
                project,
                target_idx,
                inputs,
                "DEGREES",
                "missing DEGREES input",
                "failed to parse DEGREES input in MotionTurnLeft block",
            )?;
            Ok(MotionStmt::TurnLeft { degrees })
        }
        BlockOpCodes::MotionGoTo => {
            let inputs = block_inputs(block, "missing inputs in MotionGoTo block")?;
            let target = required_expr_input(
                project,
                target_idx,
                inputs,
                "TO",
                "missing TO input",
                "failed to parse TO input in MotionGoTo block",
            )?;
            Ok(MotionStmt::Goto { to: target })
        }
        BlockOpCodes::MotionGoToXY => {
            let inputs = block_inputs(block, "missing inputs in MotionGoToXY block")?;
            let x = required_expr_input(
                project,
                target_idx,
                inputs,
                "X",
                "missing X input",
                "failed to parse X input in MotionGoToXY block",
            )?;
            let y = required_expr_input(
                project,
                target_idx,
                inputs,
                "Y",
                "missing Y input",
                "failed to parse Y input in MotionGoToXY block",
            )?;
            Ok(MotionStmt::GotoXY { x, y })
        }
        BlockOpCodes::MotionGlideTo => {
            let inputs = block_inputs(block, "missing inputs in MotionGlideTo block")?;
            let secs = required_expr_input(
                project,
                target_idx,
                inputs,
                "SECS",
                "missing SECS input",
                "failed to parse SECS input in MotionGlideTo block",
            )?;
            let target = required_expr_input(
                project,
                target_idx,
                inputs,
                "TO",
                "missing TO input",
                "failed to parse TO input in MotionGlideTo block",
            )?;
            Ok(MotionStmt::GlideTo { to: target, secs })
        }
        BlockOpCodes::MotionGlideSecsToXY => {
            let inputs = block_inputs(block, "missing inputs in MotionGlideSecsToXY block")?;
            let secs = required_expr_input(
                project,
                target_idx,
                inputs,
                "SECS",
                "missing SECS input",
                "failed to parse SECS input in MotionGlideSecsToXY block",
            )?;
            let x = required_expr_input(
                project,
                target_idx,
                inputs,
                "X",
                "missing X input",
                "failed to parse X input in MotionGlideSecsToXY block",
            )?;
            let y = required_expr_input(
                project,
                target_idx,
                inputs,
                "Y",
                "missing Y input",
                "failed to parse Y input in MotionGlideSecsToXY block",
            )?;
            Ok(MotionStmt::GlideToXY { secs, x, y })
        }
        BlockOpCodes::MotionPointInDirection => {
            let inputs = block_inputs(block, "missing inputs in MotionPointInDirection block")?;
            let direction = required_expr_input(
                project,
                target_idx,
                inputs,
                "DIRECTION",
                "missing DIRECTION input",
                "failed to parse DIRECTION input in MotionPointInDirection block",
            )?;
            Ok(MotionStmt::PointInDirection { direction })
        }
        BlockOpCodes::MotionPointTowards => {
            let inputs = block_inputs(block, "missing inputs in MotionPointTowards block")?;
            let towards = required_expr_input(
                project,
                target_idx,
                inputs,
                "TOWARDS",
                "missing TOWARDS input",
                "failed to parse TOWARDS input in MotionPointTowards block",
            )?;
            Ok(MotionStmt::PointToTowards { towards })
        }
        BlockOpCodes::MotionChangeXBy => {
            let inputs = block_inputs(block, "missing inputs in MotionChangeXBy block")?;
            let dx = required_expr_input(
                project,
                target_idx,
                inputs,
                "DX",
                "missing DX input",
                "failed to parse DX input in MotionChangeXBy block",
            )?;
            Ok(MotionStmt::ChangeXBy { dx })
        }
        BlockOpCodes::MotionSetX => {
            let inputs = block_inputs(block, "missing inputs in MotionSetX block")?;
            let x = required_expr_input(
                project,
                target_idx,
                inputs,
                "X",
                "missing X input",
                "failed to parse X input in MotionSetX block",
            )?;
            Ok(MotionStmt::SetX { x })
        }
        BlockOpCodes::MotionChangeYBy => {
            let inputs = block_inputs(block, "missing inputs in MotionChangeYBy block")?;
            let dy = required_expr_input(
                project,
                target_idx,
                inputs,
                "DY",
                "missing DY input",
                "failed to parse DY input in MotionChangeYBy block",
            )?;
            Ok(MotionStmt::ChangeYBy { dy })
        }
        BlockOpCodes::MotionSetY => {
            let inputs = block_inputs(block, "missing inputs in MotionSetY block")?;
            let y = required_expr_input(
                project,
                target_idx,
                inputs,
                "Y",
                "missing Y input",
                "failed to parse Y input in MotionSetY block",
            )?;
            Ok(MotionStmt::SetY { y })
        }
        BlockOpCodes::MotionIfOnEdgeBounce => Ok(MotionStmt::IfOnEdgeBounce),
        BlockOpCodes::MotionSetRotationStyle => {
            let fields = block_fields(block, "missing fields in MotionSetRotationStyle block")?;
            let field = required_field(
                fields,
                "STYLE",
                "missing STYLE field in MotionSetRotationStyle block",
            )?;
            let style = field_text(field);
            let style = match style.as_str() {
                "left-right" => RotationStyle::LeftRight,
                "all around" => RotationStyle::AllAround,
                "don't rotate" => RotationStyle::DontRotate,
                _ => return Err(ParserError::InvalidValue("unknown rotation sty;e")),
            };
            Ok(MotionStmt::SetRotationStyle { style })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
