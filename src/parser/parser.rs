use std::collections::HashMap;

use crate::{
    parser::{
        blocks::{
            control::{parse_control_expr, parse_control_stmt},
            data::{parse_data_expr, parse_data_stmt},
            event::{parse_event_expr, parse_event_stmt},
            looks::{parse_looks_expr, parse_looks_stmt},
            motion::{parse_motion_expr, parse_motion_stmt},
            operator::{parse_operator_expr, parse_operator_stmt},
            pen::{parse_pen_expr, parse_pen_stmt},
            procedures::{parse_procedures_expr, parse_procedures_stmt},
            sensing::{parse_sensing_expr, parse_sensing_stmt},
            sound::{parse_sound_expr, parse_sound_stmt},
        },
        hat::{is_hat_block, parse_hat},
        types::{Expr, Literal, ParseResult, ParserError, Stmt, Thread},
    },
    types::{
        Block, BlockAndTopLevelPrimitive, BlockKind, Input, InputPrimitiveOrReference, ScratchProject, StageOrSprite,
        primitive::{InputPrimitive, ListPrimitive, StringOrNumber, VariablePrimitive},
    },
};

fn get_target_blocks<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
) -> ParseResult<'a, &'a HashMap<String, BlockAndTopLevelPrimitive>> {
    let target = project
        .targets
        .get(target_idx)
        .ok_or(ParserError::InvalidTargetIndex(target_idx))?;
    Ok(match target {
        StageOrSprite::Stage(v) => &v.blocks,
        StageOrSprite::Sprite(v) => &v.blocks,
    })
}

fn with_context<'a, T>(result: ParseResult<'a, T>, context: impl Into<String>) -> ParseResult<'a, T> {
    let context = context.into();
    result.map_err(|err| err.context(context))
}

fn primary_input_value<'a>(input: &'a Input) -> Option<&'a InputPrimitiveOrReference> {
    match input {
        Input::V2(v) => v.1.as_ref(),
        Input::V3(v) => v.1.as_ref().or(v.2.as_ref()),
    }
}

pub fn project_parser<'a>(project: &'a ScratchProject) -> ParseResult<'a, Vec<Thread>> {
    let mut threads: Vec<Thread> = vec![];
    for (idx, sprite) in project.targets.iter().enumerate() {
        let blocks = match sprite {
            StageOrSprite::Stage(v) => &v.blocks,
            StageOrSprite::Sprite(v) => &v.blocks,
        };
        for (block_id, block_or_primitive) in blocks {
            if let BlockAndTopLevelPrimitive::Block(block) = block_or_primitive
                && is_hat_block(&block.opcode)
            {
                let hat = with_context(
                    parse_hat(project, idx, block),
                    format!("failed to parse hat block `{block_id}` at target index {idx}"),
                )?;
                let stmts = with_context(
                    parse_thread_from(project, idx, &block.next),
                    format!("failed to parse thread from hat block `{block_id}` at target index {idx}"),
                )?;
                threads.push(Thread {
                    hat: Some(hat),
                    stmts,
                    target_idx: idx,
                });
            }
        }
    }
    Ok(threads)
}

pub fn parse_thread_from<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block_id: &Option<String>,
) -> ParseResult<'a, Vec<Stmt>> {
    let blocks = get_target_blocks(project, target_idx)?;
    let mut next = block_id.clone();
    let mut stmt_vec: Vec<Stmt> = vec![];

    while let Some(next_block_id) = next {
        let block = blocks.get(&next_block_id);
        if block.is_none() {
            break;
        }
        let block = block.unwrap();

        match block {
            BlockAndTopLevelPrimitive::Block(block) => {
                let stmt = with_context(
                    parse_stmt(project, target_idx, block),
                    format!("failed to parse statement block `{next_block_id}` at target index {target_idx}"),
                )?;
                next = block.next.clone();
                stmt_vec.push(stmt);
            }
            BlockAndTopLevelPrimitive::TopLevelPrimitive(_) => {
                return Err(ParserError::UnexpectedTopLevelPrimitive(next_block_id)
                    .context(format!("while parsing thread at target index {target_idx}")));
            }
        }
    }

    Ok(stmt_vec)
}

#[allow(unreachable_code)]
pub fn parse_stmt<'a>(project: &'a ScratchProject, target_idx: usize, block: &'a Block) -> ParseResult<'a, Stmt> {
    let parsed = match block.opcode.kind() {
        BlockKind::Motion => parse_motion_stmt(project, target_idx, block).map(Stmt::Motion),
        BlockKind::Looks => parse_looks_stmt(project, target_idx, block).map(Stmt::Looks),
        BlockKind::Sound => parse_sound_stmt(project, target_idx, block).map(Stmt::Sound),
        BlockKind::Event => parse_event_stmt(project, target_idx, block).map(Stmt::Event),
        BlockKind::Control => parse_control_stmt(project, target_idx, block).map(Stmt::Control),
        BlockKind::Sensing => parse_sensing_stmt(project, target_idx, block).map(Stmt::Sensing),
        BlockKind::Operator => parse_operator_stmt(project, target_idx, block).map(Stmt::Operator),
        BlockKind::Data => parse_data_stmt(project, target_idx, block).map(Stmt::DataStmt),
        BlockKind::Pen => parse_pen_stmt(project, target_idx, block).map(Stmt::PenStmt),
        BlockKind::Procedures => parse_procedures_stmt(project, target_idx, block).map(Stmt::Procedures),
    };

    with_context(parsed, format!("failed to parse statement opcode `{}`", block.opcode))
}

pub fn parse_input<'a>(project: &'a ScratchProject, target_idx: usize, input: &'a Input) -> ParseResult<'a, Expr> {
    let primitive_reference = primary_input_value(input);
    if primitive_reference.is_some() {
        parse_expr(project, target_idx, primitive_reference.unwrap())
    } else {
        Ok(Expr::Literal(Literal::Null))
    }
}

pub fn parse_input_thread<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    input: &'a Input,
) -> ParseResult<'a, Vec<Stmt>> {
    let primitive_reference = primary_input_value(input);
    if primitive_reference.is_none() {
        return Ok(Vec::new());
    }
    let reference = match primitive_reference.unwrap() {
        InputPrimitiveOrReference::InputPrimitive(_) => {
            return Err(ParserError::InvalidValue("only accepted in Reference"));
        }
        InputPrimitiveOrReference::Reference(v) => v,
    };
    parse_thread_from(project, target_idx, &Some(reference.clone()))
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
                StringOrNumber::Number(n) => Ok(Expr::Literal(Literal::String(n.to_string()))),
                StringOrNumber::String(n) => Ok(Expr::Literal(Literal::String(n.clone()))),
            },
            InputPrimitive::BroadcastPrimitive(broadcast) => Ok(Expr::Literal(Literal::Broadcast {
                id: broadcast.2.clone(),
            })),
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
            InputPrimitive::ColorPrimitive(color) => Ok(Expr::Literal(Literal::Color { color: color.1.clone() })),
        },
        InputPrimitiveOrReference::Reference(reference) => {
            let blocks = get_target_blocks(project, target_idx)?;
            let referenced_block = blocks.get(reference).ok_or_else(|| {
                ParserError::UnknownBlock(reference.clone())
                    .context(format!("while parsing expression at target index {target_idx}"))
            })?;
            match referenced_block {
                BlockAndTopLevelPrimitive::Block(block) => with_context(
                    parse_expr_block(project, target_idx, block),
                    format!("failed to parse referenced expression block `{reference}` at target index {target_idx}"),
                ),
                BlockAndTopLevelPrimitive::TopLevelPrimitive(_) => {
                    Err(ParserError::UnexpectedTopLevelPrimitive(reference.clone())
                        .context(format!("while parsing expression at target index {target_idx}")))
                }
            }
        }
    }
}

#[allow(unreachable_code, unreachable_patterns)]
pub fn parse_expr_block<'a>(project: &'a ScratchProject, target_idx: usize, block: &'a Block) -> ParseResult<'a, Expr> {
    let parsed = match block.opcode.kind() {
        BlockKind::Control => parse_control_expr(project, target_idx, block),
        BlockKind::Data => parse_data_expr(project, target_idx, block),
        BlockKind::Event => parse_event_expr(project, target_idx, block).map(Expr::Event),
        BlockKind::Looks => parse_looks_expr(project, target_idx, block),
        BlockKind::Motion => parse_motion_expr(project, target_idx, block),
        BlockKind::Operator => parse_operator_expr(project, target_idx, block).map(Expr::Operator),
        BlockKind::Pen => parse_pen_expr(project, target_idx, block),
        BlockKind::Procedures => parse_procedures_expr(project, target_idx, block).map(Expr::Procedures),
        BlockKind::Sensing => parse_sensing_expr(project, target_idx, block),
        BlockKind::Sound => parse_sound_expr(project, target_idx, block),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    };

    with_context(parsed, format!("failed to parse expression opcode `{}`", block.opcode))
}
