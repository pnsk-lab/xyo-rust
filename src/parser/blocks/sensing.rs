use crate::{
    parser::{
        parser::parse_input,
        types::{Expr, Literal, ParseResult, ParserError, SensingExpr, TimeTarget},
    },
    types::{Block, BlockOpCodes, Fields, ScratchProject},
};

pub fn parse_sensing_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::SensingTouchingObject => {
            let inputs = block.inputs.as_ref().unwrap();
            let object_input = inputs.get("TOUCHINGOBJECTMENU").unwrap();
            let object = parse_input(project, target_idx, object_input).unwrap();
            Ok(Expr::Sensing(SensingExpr::TouchingObject {
                target: Box::new(object),
            }))
        }
        BlockOpCodes::SensingTouchingObjectMenu => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("TOUCHINGOBJECTMENU").unwrap();
            let operator = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingTouchingColor => {
            let inputs = block.inputs.as_ref().unwrap();
            let color_input = inputs.get("COLOR").unwrap();
            let color = parse_input(project, target_idx, color_input).unwrap();
            Ok(Expr::Sensing(SensingExpr::TouchingColor {
                target: Box::new(color),
            }))
        }
        BlockOpCodes::SensingColorIsTouchingColor => {
            let inputs = block.inputs.as_ref().unwrap();
            let color_input = inputs.get("COLOR").unwrap();
            let color = parse_input(project, target_idx, color_input).unwrap();
            let color2_input = inputs.get("COLOR").unwrap();
            let color2 = parse_input(project, target_idx, color2_input).unwrap();
            Ok(Expr::Sensing(SensingExpr::ColorTouchingColor {
                target: Box::new(color),
                base: Box::new(color2),
            }))
        }
        BlockOpCodes::SensingDistanceTo => {
            let inputs = block.inputs.as_ref().unwrap();
            let object_input = inputs.get("DISTANCETOMENU").unwrap();
            let object = parse_input(project, target_idx, object_input).unwrap();
            Ok(Expr::Sensing(SensingExpr::DistanceBy {
                target: Box::new(object),
            }))
        }
        BlockOpCodes::SensingDistanceToMenu => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("DISTANCETOMENU").unwrap();
            let operator = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingAnswer => Ok(Expr::Sensing(SensingExpr::Answer)),
        BlockOpCodes::SensingKeyPressed => {
            let inputs = block.inputs.as_ref().unwrap();
            let object_input = inputs.get("KEY_OPTION").unwrap();
            let object = parse_input(project, target_idx, object_input).unwrap();
            Ok(Expr::Sensing(SensingExpr::IsKeyDown {
                target: Box::new(object),
            }))
        }
        BlockOpCodes::SensingKeyOptions => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("KEY_OPTION").unwrap();
            let operator = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingMouseDown => Ok(Expr::Sensing(SensingExpr::IsMouseDown)),
        BlockOpCodes::SensingMouseX => Ok(Expr::Sensing(SensingExpr::MouseX)),
        BlockOpCodes::SensingMouseY => Ok(Expr::Sensing(SensingExpr::MouseY)),
        BlockOpCodes::SensingLoudness => Ok(Expr::Sensing(SensingExpr::Volume)),
        BlockOpCodes::SensingTimer => Ok(Expr::Sensing(SensingExpr::Timer)),
        BlockOpCodes::SensingOf => {
            let inputs = block.inputs.as_ref().unwrap();
            let object_input = inputs.get("OBJECT").unwrap();
            let object = parse_input(project, target_idx, object_input).unwrap();
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("PROPERTY").unwrap();
            let operator = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
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
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("OBJECT").unwrap();
            let operator = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            Ok(Expr::Literal(Literal::String(operator.clone())))
        }
        BlockOpCodes::SensingUsername => Ok(Expr::Sensing(SensingExpr::Username)),
        BlockOpCodes::SensingOnline => Ok(Expr::Sensing(SensingExpr::Online)),
        BlockOpCodes::SensingDaysSince2000 => Ok(Expr::Sensing(SensingExpr::Since2000Days)),
        BlockOpCodes::SensingCurrent => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("CURRENTMENU").unwrap();
            let time_target = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
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
        _ => Err(crate::parser::types::ParserError::NotHandledOp(
            block.opcode,
        )),
    }
}
