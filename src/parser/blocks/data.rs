use crate::{
    parser::{
        parser::parse_input,
        types::{DataExpr, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, Fields, ScratchProject},
};

pub fn parse_data_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, DataExpr> {
    match block.opcode {
        BlockOpCodes::DataItemOfList => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("LIST").unwrap();
            let list_id = match field {
                Fields::V1(_) => return Err(ParserError::InvalidValue("LIST Fields")),
                Fields::V2(v) => v.1.as_ref().unwrap(),
            };
            let inputs = block.inputs.as_ref().unwrap();
            let index_input = inputs.get("INDEX").unwrap();
            let idx = parse_input(project, target_idx, index_input).unwrap();
            Ok(DataExpr::GetItemOf {
                target: list_id.clone(),
                idx: Box::new(idx),
            })
        }
        BlockOpCodes::DataItemNumOfList => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("LIST").unwrap();
            let list_id = match field {
                Fields::V1(_) => return Err(ParserError::InvalidValue("LIST Fields")),
                Fields::V2(v) => v.1.as_ref().unwrap(),
            };
            let inputs = block.inputs.as_ref().unwrap();
            let content_input = inputs.get("ITEM").unwrap();
            let content = parse_input(project, target_idx, content_input).unwrap();
            Ok(DataExpr::GetItemIndex {
                target: list_id.clone(),
                content: Box::new(content),
            })
        }
        BlockOpCodes::DataLengthOfList => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("LIST").unwrap();
            let list_id = match field {
                Fields::V1(_) => return Err(ParserError::InvalidValue("LIST Fields")),
                Fields::V2(v) => v.1.as_ref().unwrap(),
            };
            Ok(DataExpr::GetLen {
                target: list_id.clone(),
            })
        }
        BlockOpCodes::DataListContainsItem => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("LIST").unwrap();
            let list_id = match field {
                Fields::V1(_) => return Err(ParserError::InvalidValue("LIST Fields")),
                Fields::V2(v) => v.1.as_ref().unwrap(),
            };
            let inputs = block.inputs.as_ref().unwrap();
            let content_input = inputs.get("ITEM").unwrap();
            let content = parse_input(project, target_idx, content_input).unwrap();
            Ok(DataExpr::IsInclude {
                target: list_id.clone(),
                content: Box::new(content),
            })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
