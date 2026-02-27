use crate::{
    parser::{
        parser::parse_input,
        types::{DataExpr, DataStmt, Expr, Literal, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, Fields, Input, ScratchProject},
};
use std::collections::HashMap;

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

fn get_list_id<'a>(
    fields: &'a HashMap<String, Fields>,
    missing_field_error: &'static str,
) -> ParseResult<'a, &'a String> {
    let field = fields
        .get("LIST")
        .ok_or(ParserError::InvalidValue(missing_field_error))?;
    match field {
        Fields::V1(_) => Err(ParserError::InvalidValue("LIST Fields")),
        Fields::V2(v) => {
            v.1.as_ref()
                .ok_or(ParserError::InvalidValue("missing LIST id"))
        }
    }
}

fn parse_required_input<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    inputs: &'a HashMap<String, Input>,
    key: &'static str,
    missing_error: &'static str,
    parse_error: &'static str,
) -> ParseResult<'a, crate::parser::types::Expr> {
    let input = inputs
        .get(key)
        .ok_or(ParserError::InvalidValue(missing_error))?;
    parse_input(project, target_idx, input).map_err(|err| err.context(parse_error))
}

pub fn parse_data_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, Expr> {
    match block.opcode {
        BlockOpCodes::DataItemOfList => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataItemOfList block",
            ))?;
            let list_id = get_list_id(fields, "missing LIST field in DataItemOfList block")?;
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataItemOfList block",
            ))?;
            let idx = parse_required_input(
                project,
                target_idx,
                inputs,
                "INDEX",
                "missing INDEX input",
                "failed to parse INDEX input in DataItemOfList block",
            )?;
            Ok(Expr::Data(DataExpr::GetItemOf {
                target: list_id.clone(),
                idx: Box::new(idx),
            }))
        }
        BlockOpCodes::DataItemNumOfList => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataItemNumOfList block",
            ))?;
            let list_id = get_list_id(fields, "missing LIST field in DataItemNumOfList block")?;
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataItemNumOfList block",
            ))?;
            let content = parse_required_input(
                project,
                target_idx,
                inputs,
                "ITEM",
                "missing ITEM input",
                "failed to parse ITEM input in DataItemNumOfList block",
            )?;
            Ok(Expr::Data(DataExpr::GetItemIndex {
                target: list_id.clone(),
                content: Box::new(content),
            }))
        }
        BlockOpCodes::DataLengthOfList => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataLengthOfList block",
            ))?;
            let list_id = get_list_id(fields, "missing LIST field in DataLengthOfList block")?;
            Ok(Expr::Data(DataExpr::GetLen {
                target: list_id.clone(),
            }))
        }
        BlockOpCodes::DataListContainsItem => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataListContainsItem block",
            ))?;
            let list_id = get_list_id(fields, "missing LIST field in DataListContainsItem block")?;
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataListContainsItem block",
            ))?;
            let content = parse_required_input(
                project,
                target_idx,
                inputs,
                "ITEM",
                "missing ITEM input",
                "failed to parse ITEM input in DataListContainsItem block",
            )?;
            Ok(Expr::Data(DataExpr::IsInclude {
                target: list_id.clone(),
                content: Box::new(content),
            }))
        }
        BlockOpCodes::DataVariable => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataVariable block",
            ))?;
            let variable_id =
                get_variable_id(fields, "missing VARIABLE field in DataVariable block")?;
            Ok(Expr::Literal(Literal::Variable {
                target: variable_id.clone(),
            }))
        }
        BlockOpCodes::DataListContents => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataListContents block",
            ))?;
            let list_id = get_list_id(fields, "missing VARIABLE field in DataVariable block")?;
            Ok(Expr::Literal(Literal::List {
                target: list_id.clone(),
            }))
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}

pub fn parse_data_stmt<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, DataStmt> {
    match block.opcode {
        BlockOpCodes::DataSetVariableTo => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataSetVariableTo block",
            ))?;
            let value = inputs
                .get("VALUE")
                .ok_or(ParserError::InvalidValue("missing VALUE input"))?;
            let value = parse_input(project, target_idx, value)
                .map_err(|err| err.context("failed to parse VALUE in DataSetVariableTo block"))?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataSetVariableTo block",
            ))?;
            let variable =
                get_variable_id(fields, "missing VARIABLE field in DataSetVariableTo block")?;
            Ok(DataStmt::SetVariable {
                value,
                variable: (variable.clone()),
            })
        }
        BlockOpCodes::DataChangeVariableBy => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataChangeVariableBy block",
            ))?;
            let value = inputs
                .get("VALUE")
                .ok_or(ParserError::InvalidValue("missing VALUE input"))?;
            let value = parse_input(project, target_idx, value).map_err(|err| {
                err.context("failed to parse VALUE in DataChangeVariableBy block")
            })?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataChangeVariableBy block",
            ))?;
            let variable = get_variable_id(
                fields,
                "missing VARIABLE field in DataChangeVariableBy block",
            )?;
            Ok(DataStmt::ChangeVariableBy {
                value,
                variable: (variable.clone()),
            })
        }
        BlockOpCodes::DataShowVariable => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataShowVariable block",
            ))?;
            let variable =
                get_variable_id(fields, "missing VARIABLE field in DataShowVariable block")?;
            Ok(DataStmt::ShowVariable {
                variable: (variable.clone()),
            })
        }
        BlockOpCodes::DataHideVariable => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataHideVariable block",
            ))?;
            let variable =
                get_variable_id(fields, "missing VARIABLE field in DataHideVariable block")?;
            Ok(DataStmt::HideVariable {
                variable: (variable.clone()),
            })
        }
        BlockOpCodes::DataAddToList => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataAddToList block",
            ))?;
            let item = inputs
                .get("ITEM")
                .ok_or(ParserError::InvalidValue("missing ITEM input"))?;
            let item = parse_input(project, target_idx, item)
                .map_err(|err| err.context("failed to parse ITEM in DataAddToList block"))?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataAddToList block",
            ))?;
            let list = get_list_id(fields, "missing LIST field in DataAddToList block")?;
            Ok(DataStmt::AddToList {
                item,
                list: (list.clone()),
            })
        }
        BlockOpCodes::DataDeleteOfList => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataDeleteOfList block",
            ))?;
            let idx = inputs
                .get("INDEX")
                .ok_or(ParserError::InvalidValue("missing INDEX input"))?;
            let idx = parse_input(project, target_idx, idx)
                .map_err(|err| err.context("failed to parse INDEX in DataDeleteOfList block"))?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataDeleteOfList block",
            ))?;
            let list = get_list_id(fields, "missing LIST field in DataDeleteOfList block")?;
            Ok(DataStmt::DeleteOfList {
                idx,
                list: (list.clone()),
            })
        }
        BlockOpCodes::DataDeleteAllOfList => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataDeleteAllOfList block",
            ))?;
            let list = get_list_id(fields, "missing LIST field in DataDeleteAllOfList block")?;
            Ok(DataStmt::DeleteAllOfList {
                list: (list.clone()),
            })
        }
        BlockOpCodes::DataInsertAtList => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataInsertAtList block",
            ))?;
            let idx = inputs
                .get("INDEX")
                .ok_or(ParserError::InvalidValue("missing INDEX input"))?;
            let idx = parse_input(project, target_idx, idx)
                .map_err(|err| err.context("failed to parse INDEX in DataInsertAtList block"))?;
            let item = inputs
                .get("ITEM")
                .ok_or(ParserError::InvalidValue("missing ITEM input"))?;
            let item = parse_input(project, target_idx, item)
                .map_err(|err| err.context("failed to parse ITEM in DataInsertAtList block"))?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataInsertAtList block",
            ))?;
            let list = get_list_id(fields, "missing LIST field in DataInsertAtList block")?;
            Ok(DataStmt::InsertAtList {
                idx,
                item,
                list: (list.clone()),
            })
        }
        BlockOpCodes::DataReplaceItemOfList => {
            let inputs = block.inputs.as_ref().ok_or(ParserError::InvalidValue(
                "missing inputs in DataReplaceItemOfList block",
            ))?;
            let idx = inputs
                .get("INDEX")
                .ok_or(ParserError::InvalidValue("missing INDEX input"))?;
            let idx = parse_input(project, target_idx, idx).map_err(|err| {
                err.context("failed to parse INDEX in DataReplaceItemOfList block")
            })?;
            let item = inputs
                .get("ITEM")
                .ok_or(ParserError::InvalidValue("missing ITEM input"))?;
            let item = parse_input(project, target_idx, item).map_err(|err| {
                err.context("failed to parse ITEM in DataReplaceItemOfList block")
            })?;
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataReplaceItemOfList block",
            ))?;
            let list = get_list_id(fields, "missing LIST field in DataReplaceItemOfList block")?;
            Ok(DataStmt::ReplaceAtList {
                idx,
                item,
                list: (list.clone()),
            })
        }
        BlockOpCodes::DataShowList => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataShowList block",
            ))?;
            let list = get_list_id(fields, "missing List field in DataShowList block")?;
            Ok(DataStmt::ShowList {
                list: (list.clone()),
            })
        }
        BlockOpCodes::DataHideList => {
            let fields = block.fields.as_ref().ok_or(ParserError::InvalidValue(
                "missing fields in DataHideList block",
            ))?;
            let list = get_list_id(fields, "missing List field in DataHideList block")?;
            Ok(DataStmt::HideList {
                list: (list.clone()),
            })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
