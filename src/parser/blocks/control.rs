use std::collections::HashMap;

use crate::{
    parser::{
        parser::{parse_input, parse_input_thread},
        types::{
            ControlExpr, ControlStmt, Expr, Literal, ParseResult, ParserError, Stmt, StopOption,
        },
    },
    types::{Block, BlockOpCodes, Fields, ScratchProject},
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
fn get_variable_id<'a>(
    fields: &'a HashMap<String, Fields>,
    missing_field_error: &'static str,
) -> ParseResult<'a, &'a String> {
    let field = fields
        .get("VARIABLE")
        .ok_or(ParserError::InvalidValue(missing_field_error))?;
    match field {
        Fields::V1(_) => Err(ParserError::InvalidValue("VARIABLE Fields")),
        Fields::V2(v) => {
            v.1.as_ref()
                .ok_or(ParserError::InvalidValue("missing VARIABLE id"))
        }
    }
}

pub fn parse_control_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::ControlCreateCloneOfMenu => {
            let fields = block_fields(block, "missing fields in ControlCreateCloneOfMenu block")?;
            let field = required_field(
                fields,
                "CLONE_OPTION",
                "missing NUMBER_NAME field in ControlCreateCloneOfMenu block",
            )?;
            let clone_option = field_text(field);
            Ok(Expr::Literal(Literal::String(clone_option.clone())))
        }
        BlockOpCodes::ControlGetCounter => Ok(Expr::Control(ControlExpr::GetCounter)),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
pub fn parse_control_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, ControlStmt> {
    match block.opcode {
        BlockOpCodes::ControlWait => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in ControlWait block",
            ))?;
            let duration_input = inputs
                .get("DURATION")
                .ok_or(ParserError::InvalidValue("missing DURATION input"))?;
            let duration = parse_input(project, target_idx, duration_input).map_err(|err| {
                err.context("failed to parse DURATION input in ControlWait block")
            })?;
            Ok(ControlStmt::Wait { duration })
        }
        BlockOpCodes::ControlRepeat => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in ControlRepeat block",
            ))?;
            let times_input = inputs
                .get("TIMES")
                .ok_or(ParserError::InvalidValue("missing TIMES input"))?;
            let times = parse_input(project, target_idx, times_input)
                .map_err(|err| err.context("failed to parse TIMES input in ControlRepeat block"))?;
            let substack_input = inputs.get("SUBSTACK");

            let substack: Option<Vec<Stmt>> = if let Some(substack_input) = substack_input {
                Some(
                    parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                        err.context("failed to parse SUBSTACK input in ControlRepeat block")
                    })?,
                )
            } else {
                None
            };
            Ok(ControlStmt::Repeat { times, substack })
        }
        BlockOpCodes::ControlForever => {
            let inputs = block.inputs.as_ref();

            let substack: Option<Vec<Stmt>> = if inputs.is_some()
                && let Some(substack_input) = inputs.unwrap().get("SUBSTACK")
            {
                Some(
                    parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                        err.context("failed to parse SUBSTACK input in ControlForever block")
                    })?,
                )
            } else {
                None
            };
            Ok(ControlStmt::Forever { substack })
        }
        BlockOpCodes::ControlIf => {
            let inputs = block.inputs.as_ref();

            let (condition, substack) = if inputs.is_some() {
                let condition_input = inputs.unwrap().get("CONDITION");
                let condition = if let Some(condition_input) = condition_input {
                    Some(
                        parse_input(project, target_idx, condition_input).map_err(|err| {
                            err.context("failed to parse CONDITION input in ControlIf block")
                        })?,
                    )
                } else {
                    None
                };
                let substack_input = inputs.unwrap().get("SUBSTACK");
                let substack: Option<Vec<Stmt>> = if let Some(substack_input) = substack_input {
                    Some(
                        parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                            err.context("failed to parse SUBSTACK input in ControlIf block")
                        })?,
                    )
                } else {
                    None
                };
                (condition, substack)
            } else {
                (None, None)
            };

            Ok(ControlStmt::If {
                condition,
                substack,
            })
        }
        BlockOpCodes::ControlIfElse => {
            let inputs = block.inputs.as_ref();

            let (condition, substack, substack2) = if inputs.is_some() {
                let condition_input = inputs.unwrap().get("CONDITION");
                let condition = if let Some(condition_input) = condition_input {
                    Some(
                        parse_input(project, target_idx, condition_input).map_err(|err| {
                            err.context("failed to parse CONDITION input in ControlIfElse block")
                        })?,
                    )
                } else {
                    None
                };
                let substack_input = inputs.unwrap().get("SUBSTACK");
                let substack: Option<Vec<Stmt>> = if let Some(substack_input) = substack_input {
                    Some(
                        parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                            err.context("failed to parse SUBSTACK input in ControlIfElse block")
                        })?,
                    )
                } else {
                    None
                };
                let substack2_input = inputs.unwrap().get("SUBSTACK2");
                let substack2: Option<Vec<Stmt>> = if let Some(substack2_input) = substack2_input {
                    Some(
                        parse_input_thread(project, target_idx, substack2_input).map_err(
                            |err| {
                                err.context(
                                    "failed to parse SUBSTACK2 input in ControlIfElse block",
                                )
                            },
                        )?,
                    )
                } else {
                    None
                };
                (condition, substack, substack2)
            } else {
                (None, None, None)
            };
            Ok(ControlStmt::IfElse {
                condition,
                substack,
                substack2,
            })
        }
        BlockOpCodes::ControlWaitUntil => {
            let inputs = block.inputs.as_ref();

            let condition = if inputs.is_some() {
                if let Some(condition_input) = inputs.unwrap().get("CONDITION") {
                    Some(
                        parse_input(project, target_idx, condition_input).map_err(|err| {
                            err.context("failed to parse CONDITION input in ControlWaitUntil block")
                        })?,
                    )
                } else {
                    None
                }
            } else {
                None
            };

            Ok(ControlStmt::WaitUntil { condition })
        }
        BlockOpCodes::ControlRepeatUntil => {
            let inputs = block.inputs.as_ref();

            let (condition, substack) = if inputs.is_some() {
                let condition_input = inputs.unwrap().get("CONDITION");
                let condition = if let Some(condition_input) = condition_input {
                    Some(
                        parse_input(project, target_idx, condition_input).map_err(|err| {
                            err.context(
                                "failed to parse CONDITION input in ControlRepeatUntil block",
                            )
                        })?,
                    )
                } else {
                    None
                };
                let substack_input = inputs.unwrap().get("SUBSTACK");
                let substack: Option<Vec<Stmt>> = if let Some(substack_input) = substack_input {
                    Some(
                        parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                            err.context(
                                "failed to parse SUBSTACK input in ControlRepeatUntil block",
                            )
                        })?,
                    )
                } else {
                    None
                };
                (condition, substack)
            } else {
                (None, None)
            };

            Ok(ControlStmt::RepeatUntil {
                condition,
                substack,
            })
        }
        BlockOpCodes::ControlWhile => {
            let inputs = block.inputs.as_ref();

            let (condition, substack) = if inputs.is_some() {
                let condition: Option<Expr> =
                    if let Some(condition_input) = inputs.unwrap().get("CONDITION") {
                        Some(
                            parse_input(project, target_idx, condition_input).map_err(|err| {
                                err.context("failed to parse CONDITION input in ControlWhile block")
                            })?,
                        )
                    } else {
                        None
                    };

                let substack: Option<Vec<Stmt>> = if let Some(substack_input) =
                    inputs.unwrap().get("SUBSTACK")
                {
                    Some(
                        parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                            err.context("failed to parse SUBSTACK input in ControlWhile block")
                        })?,
                    )
                } else {
                    None
                };
                (condition, substack)
            } else {
                (None, None)
            };
            Ok(ControlStmt::RepeatWhile {
                condition,
                substack,
            })
        }
        BlockOpCodes::ControlAllAtOnce => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in ControlAllAtOnce block",
            ))?;
            let substack_input = inputs.get("SUBSTACK");
            let substack: Option<Vec<Stmt>> = if let Some(substack_input) = substack_input {
                Some(
                    parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                        err.context("failed to parse SUBSTACK input in ControlWhile block")
                    })?,
                )
            } else {
                None
            };
            Ok(ControlStmt::AllAtOnce { substack })
        }
        BlockOpCodes::ControlCreateCloneOf => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in ControlCreateCloneOf block",
            ))?;
            let clone_option = inputs
                .get("CLONE_OPTION")
                .ok_or(ParserError::InvalidValue("missing CLONE_OPTION input"))?;
            let clone_option = parse_input(project, target_idx, clone_option).map_err(|err| {
                err.context("failed to parse CLONE_OPTION input in ControlCreateCloneOf block")
            })?;
            Ok(ControlStmt::CreateCloneOf { clone_option })
        }
        BlockOpCodes::ControlDeleteThisClone => Ok(ControlStmt::DeleteThisClone),
        BlockOpCodes::ControlStop => {
            let fields = block_fields(block, "missing fields in ControlStop block")?;
            let field = required_field(
                fields,
                "STOP_OPTION",
                "missing NUMBER_NAME field in ControlStop block",
            )?;
            let stop_option = field_text(field);
            let stop_option = match stop_option.as_str() {
                "all" => StopOption::All,
                "this script" => StopOption::ThisScript,
                "other scripts in sprite" => StopOption::OtherScrriptInSprite,
                _ => {
                    return Err(ParserError::InvalidValue(
                        "Stop option has to be all, this script or other scripts in sprite",
                    ));
                }
            };
            Ok(ControlStmt::Stop {
                option: stop_option,
            })
        }
        BlockOpCodes::ControlForEach => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in ControlForEach block",
            ))?;
            let substack_input = inputs.get("SUBSTACK");
            let substack: Option<Vec<Stmt>> = if let Some(substack_input) = substack_input {
                Some(
                    parse_input_thread(project, target_idx, substack_input).map_err(|err| {
                        err.context("failed to parse SUBSTACK input in ControlForEach block")
                    })?,
                )
            } else {
                None
            };
            let value = inputs
                .get("VALUE")
                .ok_or(ParserError::InvalidValue("missing VALUE input"))?;
            let value = parse_input(project, target_idx, value).map_err(|err| {
                err.context("failed to parse VALUE input in ControlForEach block")
            })?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataSetVariableTo block",
            ))?;
            let variable =
                get_variable_id(fields, "missing VARIABLE field in ControlForEach block")?;
            Ok(ControlStmt::ForEach {
                variable: variable.clone(),
                value,
                substack,
            })
        }
        BlockOpCodes::ControlIncrCounter => Ok(ControlStmt::IncrCounter),
        BlockOpCodes::ControlClearCounter => Ok(ControlStmt::ClearCounter),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
