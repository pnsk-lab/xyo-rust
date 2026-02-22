use crate::{
    parser::types::{CostumeStatueTarget, LooksExpr, ParseResult, ParserError},
    types::{Block, BlockOpCodes, Fields, ScratchProject},
};

pub fn parse_looks_expr<'a>(
    _: &'a ScratchProject,
    _: usize,
    block: &'a Block,
) -> ParseResult<'a, LooksExpr> {
    match block.opcode {
        BlockOpCodes::LooksCostumeNumberName => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("NUMBER_NAME").unwrap();
            let number_or_name = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            let number_or_name = if number_or_name == "number" {
                CostumeStatueTarget::Number
            } else if number_or_name == "name" {
                CostumeStatueTarget::Name
            } else {
                return Err(ParserError::InvalidValue(
                    "NUMBER_NAME can supported in number or name",
                ));
            };
            Ok(LooksExpr::CostumeStatus {
                target: number_or_name,
            })
        }
        BlockOpCodes::LooksBackdropNumberName => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("NUMBER_NAME").unwrap();
            let number_or_name = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
            let number_or_name = if number_or_name == "number" {
                CostumeStatueTarget::Number
            } else if number_or_name == "name" {
                CostumeStatueTarget::Name
            } else {
                return Err(ParserError::InvalidValue(
                    "NUMBER_NAME can supported in number or name",
                ));
            };
            Ok(LooksExpr::BackdropStatus {
                target: number_or_name,
            })
        }
        BlockOpCodes::LooksSize => Ok(LooksExpr::Size),
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
