use crate::{
    parser::{
        blocks::{
            control::parse_control_expr, data::parse_data_expr, event::parse_event_expr,
            looks::parse_looks_expr, motion::parse_motion_expr, operator::parse_operator_expr,
            pen::parse_pen_expr, procedures::parse_procedures_expr, sensing::parse_sensing_expr,
            sound::parse_sound_expr,
        },
        hat::{is_hat_block, parse_hat},
        types::{Expr, Literal, ParseResult, ParserError},
    },
    types::{
        Block, BlockAndTopLevelPrimitive, BlockKind, Input, InputPrimitiveOrReference,
        ScratchProject, StageOrSprite,
        primitive::{InputPrimitive, ListPrimitive, StringOrNumber, VariablePrimitive},
    },
};

pub fn project_parser(project: ScratchProject) {
    for (idx, sprite) in project.targets.iter().enumerate() {
        let blocks = match &sprite {
            StageOrSprite::Stage(v) => &v.blocks,
            StageOrSprite::Sprite(v) => &v.blocks,
        };
        for b in blocks {
            if let BlockAndTopLevelPrimitive::Block(block) = b.1
                && is_hat_block(&block.opcode)
            {
                let hat = parse_hat(&project, idx, block).unwrap();
                println!("{:?}", hat);
            }
        }
    }
}

pub fn parse_input<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    input: &'a Input,
) -> ParseResult<'a, Expr> {
    let primitive_reference = match input {
        Input::V2(v) => &v.1,
        Input::V3(v) => &v.1,
    };
    parse_expr(project, target_idx, primitive_reference)
}
pub fn parse_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    input: &'a InputPrimitiveOrReference,
) -> ParseResult<'a, Expr> {
    match input {
        InputPrimitiveOrReference::InputPrimitive(primitive) => match primitive {
            InputPrimitive::NumPrimitive(num) => match &num.1 {
                StringOrNumber::Number(n) => Ok(Expr::Literal(Literal::Number(n.to_string()))),
                StringOrNumber::String(n) => Ok(Expr::Literal(Literal::Number(n.clone()))),
            },
            InputPrimitive::TextPrimitive(num) => match &num.1 {
                StringOrNumber::Number(n) => Ok(Expr::Literal(Literal::Number(n.to_string()))),
                StringOrNumber::String(n) => Ok(Expr::Literal(Literal::Number(n.clone()))),
            },
            InputPrimitive::BroadcastPrimitive(broadcast) => {
                Ok(Expr::Literal(Literal::Broadcast {
                    id: broadcast.2.clone(),
                }))
            }
            InputPrimitive::VariablePrimitive(var) => Ok(Expr::Literal(Literal::Variable {
                target: match var {
                    VariablePrimitive::V3(v) => v.2.clone(),
                    VariablePrimitive::V5(v) => v.2.clone(),
                },
            })),
            InputPrimitive::ListPrimitive(var) => Ok(Expr::Literal(Literal::List {
                target: match var {
                    ListPrimitive::V3(v) => v.2.clone(),
                    ListPrimitive::V5(v) => v.2.clone(),
                },
            })),
            InputPrimitive::ColorPrimitive(color) => Ok(Expr::Literal(Literal::Color {
                color: color.1.clone(),
            })),
        },
        InputPrimitiveOrReference::Reference(reference) => {
            let target = &project.targets[target_idx];
            let referenced_block = match target {
                StageOrSprite::Sprite(sprite) => sprite.blocks.get(reference),
                StageOrSprite::Stage(stage) => stage.blocks.get(reference),
            };
            if referenced_block.is_none() {
                return Err(ParserError::UnknownBlock(reference.clone()));
            }
            let referenced_block = referenced_block.unwrap();
            match referenced_block {
                BlockAndTopLevelPrimitive::Block(block) => {
                    parse_expr_block(project, target_idx, block)
                }
                BlockAndTopLevelPrimitive::TopLevelPrimitive(_) => {
                    Err(ParserError::UnknownBlock(reference.clone()))
                }
            }
        }
    }
}
#[allow(unreachable_code, unreachable_patterns)]
pub fn parse_expr_block<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode.kind() {
        BlockKind::Control => {
            let parse_result = parse_control_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Control(parse_result.unwrap()))
        }
        BlockKind::Data => {
            let parse_result = parse_data_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Data(parse_result.unwrap()))
        }
        BlockKind::Event => {
            let parse_result = parse_event_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Event(parse_result.unwrap()))
        }
        BlockKind::Looks => {
            let parse_result = parse_looks_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Looks(parse_result.unwrap()))
        }
        BlockKind::Motion => {
            let parse_result = parse_motion_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Motion(parse_result.unwrap()))
        }
        BlockKind::Operator => {
            let parse_result = parse_operator_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Operator(parse_result.unwrap()))
        }
        BlockKind::Pen => {
            let parse_result = parse_pen_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Pen(parse_result.unwrap()))
        }
        BlockKind::Procedures => {
            let parse_result = parse_procedures_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Procedures(parse_result.unwrap()))
        }
        BlockKind::Sensing => {
            let parse_result = parse_sensing_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(parse_result.unwrap())
        }
        BlockKind::Sound => {
            let parse_result = parse_sound_expr(project, target_idx, block);
            if parse_result.is_err() {
                return Err(parse_result.err().unwrap());
            }
            Ok(Expr::Sound(parse_result.unwrap()))
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
