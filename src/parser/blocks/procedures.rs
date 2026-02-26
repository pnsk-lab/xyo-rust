use std::collections::HashMap;

use crate::{
    parser::{
        parser::parse_input,
        types::{
            Argument, Expr, ParseResult, ParserError, ProceduresExpr, ProceduresPrototypeStruct,
            ProceduresStmt,
        },
    },
    types::{
        Block, BlockOpCodes, Fields, Input, Mutation, ScratchProject, StringOrStringArray,
        WarpValue, primitive::StringOrNumber,
    },
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
    key: &str,
    missing_input_error: &'a str,
    parse_error: &str,
) -> ParseResult<'a, Expr> {
    let input = inputs
        .get(key)
        .ok_or(ParserError::InvalidValue(missing_input_error))?;
    parse_input(project, target_idx, input).map_err(|err| err.context(parse_error))
}

pub fn parse_procedures_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, ProceduresExpr> {
    match block.opcode {
        BlockOpCodes::ProceduresPrototype => {
            let mutation = block.mutation.as_ref().ok_or(ParserError::InvalidValue(
                "missing mutation in ProceduresPrototype",
            ))?;
            match mutation {
                Mutation::MutationProceduresPrototype(v) => {
                    let argument_ids: Vec<String> =
                        serde_json::from_str::<Vec<String>>(&v.argumentids).map_err(|_| {
                            ParserError::InvalidValue("JSON parsing error on mutation.argumentids")
                        })?;
                    let argument_names: Vec<String> =
                        serde_json::from_str::<Vec<String>>(&v.argumentnames).map_err(|_| {
                            ParserError::InvalidValue(
                                "JSON parsing error on mutation.argumentnames",
                            )
                        })?;
                    let argument_defaults: Vec<StringOrNumber> = serde_json::from_str::<
                        Vec<StringOrNumber>,
                    >(
                        &v.argumentdefaults
                    )
                    .map_err(|_| {
                        ParserError::InvalidValue("JSON parsing error on mutation.argumentdefaults")
                    })?;
                    let warp = v.warp.clone().unwrap_or(WarpValue::Bool(false));
                    let warp = match warp {
                        WarpValue::Bool(v) => v,
                        WarpValue::String(v) => match v.as_str() {
                            "true" => true,
                            "false" => false,
                            _ => {
                                return Err(ParserError::InvalidValue(
                                    "warp value must be 'true' or 'false'",
                                ));
                            }
                        },
                        _ => {
                            return Err(ParserError::InvalidValue(
                                "warp value must be boolean or 'true' or 'false'",
                            ));
                        }
                    };
                    if argument_ids.len() != argument_names.len() {
                        return Err(ParserError::InvalidValue("arguments len is not matched"));
                    }
                    let mut defaults_iter = argument_defaults.into_iter();
                    let arguments: Vec<Argument> = argument_ids
                        .into_iter()
                        .zip(argument_names.into_iter())
                        .map(|(id, name)| Argument {
                            id,
                            name,
                            default: defaults_iter
                                .next()
                                .unwrap_or(StringOrNumber::String(String::new())),
                        })
                        .collect();
                    Ok(ProceduresExpr::ProceduresPrototype {
                        prototype: ProceduresPrototypeStruct {
                            warp,
                            arguments,
                            proccode: v.proccode.clone(),
                        },
                    })
                }
                _ => Err(ParserError::InvalidValue(
                    "Mutation has to be MutationProceduresPrototype",
                )),
            }
        }
        BlockOpCodes::ArgumentReporterBoolean => {
            let fields = block_fields(block, "missing fields in ArgumentReporterBoolean block")?;
            let field = required_field(
                fields,
                "VALUE",
                "missing VALUE field in ArgumentReporterBoolean block",
            )?;
            let name = field_text(&field);
            Ok(ProceduresExpr::ArgumentReporterBoolean { name: name.clone() })
        }
        BlockOpCodes::ArgumentReporterStringNumber => {
            let fields = block_fields(
                block,
                "missing fields in ArgumentReporterStringNumber block",
            )?;
            let field = required_field(
                fields,
                "VALUE",
                "missing VALUE field in ArgumentReporterStringNumber block",
            )?;
            let name = field_text(&field);
            Ok(ProceduresExpr::ArgumentReporterStringNumber { name: name.clone() })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}

pub fn parse_procedures_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, ProceduresStmt> {
    match block.opcode {
        BlockOpCodes::ProceduresCall => {
            let mutation = block.mutation.as_ref().ok_or(ParserError::InvalidValue(
                "missing mutation in ProceduresPrototype",
            ))?;
            match mutation {
                Mutation::MutationProceduresCall(v) => {
                    let input_field =
                        block_inputs(block, "missing inputs in MutationProceduresCall block")?;
                    let argument_ids: Vec<String> = match &v.argumentids {
                        StringOrStringArray::String(v) => serde_json::from_str::<Vec<String>>(&v)
                            .map_err(|_| {
                            ParserError::InvalidValue("JSON parsing error on mutation.argumentids")
                        })?,
                        StringOrStringArray::StringArray(v) => v.clone(),
                    };
                    let mut inputs: HashMap<String, Expr> = HashMap::new();
                    for i in argument_ids {
                        let input = required_expr_input(
                            project,
                            target_idx,
                            input_field,
                            i.clone().as_str(),
                            "missing procedure input in MutationProceduresCall block",
                            "failed to parse procedure input in MutationProceduresCall block",
                        )?;
                        inputs.insert(i.clone(), input);
                    }
                    Ok(ProceduresStmt::ProceduresCall {
                        proccode: v.proccode.clone(),
                        inputs: inputs,
                    })
                }
                _ => Err(ParserError::InvalidValue(
                    "Mutation has to be MutationProceduresPrototype",
                )),
            }
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
