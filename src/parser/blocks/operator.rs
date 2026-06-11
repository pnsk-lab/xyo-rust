use std::collections::HashMap;

use crate::{
    parser::{
        parser::parse_input,
        types::{CalcOp, Expr, OperatorExpr, OperatorStmt, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, Fields, Input, ScratchProject},
};

fn block_inputs<'a>(
    block: &'a Block,
    missing_inputs_error: &'static str,
) -> ParseResult<'a, &'a HashMap<String, Input>> {
    block
        .inputs
        .as_ref()
        .ok_or(ParserError::InvalidValue(missing_inputs_error))
}

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
    fields.get(key).ok_or(ParserError::InvalidValue(missing_field_error))
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
    let input = inputs.get(key).ok_or(ParserError::InvalidValue(missing_input_error))?;
    parse_input(project, target_idx, input).map_err(|err| err.context(parse_error))
}

fn optional_expr_input<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    inputs: &'a HashMap<String, Input>,
    key: &'static str,
    parse_error: &'static str,
) -> ParseResult<'a, Option<Box<Expr>>> {
    match inputs.get(key) {
        Some(input) => {
            let expr = parse_input(project, target_idx, input).map_err(|err| err.context(parse_error))?;
            Ok(Some(Box::new(expr)))
        }
        None => Ok(None),
    }
}

pub fn parse_operator_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, OperatorExpr> {
    match block.opcode {
        BlockOpCodes::OperatorAdd => {
            let inputs = block_inputs(block, "missing inputs in OperatorAdd block")?;
            let num1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM1",
                "missing NUM1 input",
                "failed to parse NUM1 input in OperatorAdd block",
            )?;
            let num2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM2",
                "missing NUM2 input",
                "failed to parse NUM2 input in OperatorAdd block",
            )?;
            Ok(OperatorExpr::Add {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorSubtract => {
            let inputs = block_inputs(block, "missing inputs in OperatorSubtract block")?;
            let num1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM1",
                "missing NUM1 input",
                "failed to parse NUM1 input in OperatorSubtract block",
            )?;
            let num2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM2",
                "missing NUM2 input",
                "failed to parse NUM2 input in OperatorSubtract block",
            )?;
            Ok(OperatorExpr::Sub {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorMultiply => {
            let inputs = block_inputs(block, "missing inputs in OperatorMultiply block")?;
            let num1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM1",
                "missing NUM1 input",
                "failed to parse NUM1 input in OperatorMultiply block",
            )?;
            let num2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM2",
                "missing NUM2 input",
                "failed to parse NUM2 input in OperatorMultiply block",
            )?;
            Ok(OperatorExpr::Mul {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorDivide => {
            let inputs = block_inputs(block, "missing inputs in OperatorDivide block")?;
            let num1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM1",
                "missing NUM1 input",
                "failed to parse NUM1 input in OperatorDivide block",
            )?;
            let num2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM2",
                "missing NUM2 input",
                "failed to parse NUM2 input in OperatorDivide block",
            )?;
            Ok(OperatorExpr::Div {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorRandom => {
            let inputs = block_inputs(block, "missing inputs in OperatorRandom block")?;
            let from = required_expr_input(
                project,
                target_idx,
                inputs,
                "FROM",
                "missing FROM input",
                "failed to parse FROM input in OperatorRandom block",
            )?;
            let to = required_expr_input(
                project,
                target_idx,
                inputs,
                "TO",
                "missing TO input",
                "failed to parse TO input in OperatorRandom block",
            )?;
            Ok(OperatorExpr::Random {
                from: Box::new(from),
                to: Box::new(to),
            })
        }
        BlockOpCodes::OperatorGt => {
            let inputs = block_inputs(block, "missing inputs in OperatorGt block")?;
            let operand1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND1",
                "missing OPERAND1 input",
                "failed to parse OPERAND1 input in OperatorGt block",
            )?;
            let operand2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND2",
                "missing OPERAND2 input",
                "failed to parse OPERAND2 input in OperatorGt block",
            )?;
            Ok(OperatorExpr::GreaterThan {
                left: Box::new(operand1),
                right: Box::new(operand2),
            })
        }
        BlockOpCodes::OperatorLt => {
            let inputs = block_inputs(block, "missing inputs in OperatorLt block")?;
            let operand1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND1",
                "missing OPERAND1 input",
                "failed to parse OPERAND1 input in OperatorLt block",
            )?;
            let operand2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND2",
                "missing OPERAND2 input",
                "failed to parse OPERAND2 input in OperatorLt block",
            )?;
            Ok(OperatorExpr::LessThan {
                left: Box::new(operand1),
                right: Box::new(operand2),
            })
        }
        BlockOpCodes::OperatorEquals => {
            let inputs = block_inputs(block, "missing inputs in OperatorEquals block")?;
            let operand1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND1",
                "missing OPERAND1 input",
                "failed to parse OPERAND1 input in OperatorEquals block",
            )?;
            let operand2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND2",
                "missing OPERAND2 input",
                "failed to parse OPERAND2 input in OperatorEquals block",
            )?;
            Ok(OperatorExpr::Eq {
                left: Box::new(operand1),
                right: Box::new(operand2),
            })
        }
        BlockOpCodes::OperatorAnd => {
            let inputs = block_inputs(block, "missing inputs in OperatorAnd block")?;
            let operand1 = optional_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND1",
                "failed to parse OPERAND1 input in OperatorAnd block",
            )?;
            let operand2 = optional_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND2",
                "failed to parse OPERAND2 input in OperatorAnd block",
            )?;
            Ok(OperatorExpr::And {
                left: operand1,
                right: operand2,
            })
        }
        BlockOpCodes::OperatorOr => {
            let inputs = block_inputs(block, "missing inputs in OperatorOr block")?;
            let operand1 = optional_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND1",
                "failed to parse OPERAND1 input in OperatorOr block",
            )?;
            let operand2 = optional_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND2",
                "failed to parse OPERAND2 input in OperatorOr block",
            )?;
            Ok(OperatorExpr::Or {
                left: operand1,
                right: operand2,
            })
        }
        BlockOpCodes::OperatorNot => {
            let inputs = block_inputs(block, "missing inputs in OperatorNot block")?;
            let operand = optional_expr_input(
                project,
                target_idx,
                inputs,
                "OPERAND",
                "failed to parse OPERAND input in OperatorNot block",
            )?;
            Ok(OperatorExpr::Not { target: operand })
        }
        BlockOpCodes::OperatorJoin => {
            let inputs = block_inputs(block, "missing inputs in OperatorJoin block")?;
            let string1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRING1",
                "missing STRING1 input",
                "failed to parse STRING1 input in OperatorJoin block",
            )?;
            let string2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRING2",
                "missing STRING2 input",
                "failed to parse STRING2 input in OperatorJoin block",
            )?;
            Ok(OperatorExpr::Join {
                left: Box::new(string1),
                right: Box::new(string2),
            })
        }
        BlockOpCodes::OperatorLetterOf => {
            let inputs = block_inputs(block, "missing inputs in OperatorLetterOf block")?;
            let string = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRING",
                "missing STRING input",
                "failed to parse STRING input in OperatorLetterOf block",
            )?;
            let letter = required_expr_input(
                project,
                target_idx,
                inputs,
                "LETTER",
                "missing LETTER input",
                "failed to parse LETTER input in OperatorLetterOf block",
            )?;
            Ok(OperatorExpr::Slice {
                target: Box::new(string),
                idx: Box::new(letter),
            })
        }
        BlockOpCodes::OperatorLength => {
            let inputs = block_inputs(block, "missing inputs in OperatorLength block")?;
            let string = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRING",
                "missing STRING input",
                "failed to parse STRING input in OperatorLength block",
            )?;
            Ok(OperatorExpr::Len {
                target: Box::new(string),
            })
        }
        BlockOpCodes::OperatorContains => {
            let inputs = block_inputs(block, "missing inputs in OperatorContains block")?;
            let string1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRING1",
                "missing STRING1 input",
                "failed to parse STRING1 input in OperatorContains block",
            )?;
            let string2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "STRING2",
                "missing STRING2 input",
                "failed to parse STRING2 input in OperatorContains block",
            )?;
            Ok(OperatorExpr::IsInclude {
                target: Box::new(string1),
                content: Box::new(string2),
            })
        }
        BlockOpCodes::OperatorMod => {
            let inputs = block_inputs(block, "missing inputs in OperatorMod block")?;
            let num1 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM1",
                "missing NUM1 input",
                "failed to parse NUM1 input in OperatorMod block",
            )?;
            let num2 = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM2",
                "missing NUM2 input",
                "failed to parse NUM2 input in OperatorMod block",
            )?;
            Ok(OperatorExpr::Mod {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorRound => {
            let inputs = block_inputs(block, "missing inputs in OperatorRound block")?;
            let num = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM",
                "missing NUM input",
                "failed to parse NUM input in OperatorRound block",
            )?;
            Ok(OperatorExpr::Round { target: Box::new(num) })
        }
        BlockOpCodes::OperatorMathOp => {
            let fields = block_fields(block, "missing fields in OperatorMathOp block")?;
            let field = required_field(fields, "OPERATOR", "missing OPERATOR field in OperatorMathOp block")?;
            let operator = field_text(field);
            let operator = match operator.as_str() {
                "abs" => CalcOp::Abs,
                "floor" => CalcOp::Floor,
                "ceiling" => CalcOp::Ceil,
                "sqrt" => CalcOp::Sqrt,
                "sin" => CalcOp::Sin,
                "cos" => CalcOp::Cos,
                "tan" => CalcOp::Tan,
                "asin" => CalcOp::Asin,
                "acos" => CalcOp::Acos,
                "atan" => CalcOp::Atan,
                "ln" => CalcOp::LogE,
                "log" => CalcOp::Log10,
                "e ^" => CalcOp::PowE,
                "10 ^" => CalcOp::Pow10,
                _ => return Err(ParserError::InvalidValue("unknown operator")),
            };
            let inputs = block_inputs(block, "missing inputs in OperatorMathOp block")?;
            let num = required_expr_input(
                project,
                target_idx,
                inputs,
                "NUM",
                "missing NUM input",
                "failed to parse NUM input in OperatorMathOp block",
            )?;
            Ok(OperatorExpr::Calc {
                target: Box::new(num),
                op: operator,
            })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}

pub fn parse_operator_stmt<'a>(_: &'a ScratchProject, _: usize, block: &'a Block) -> ParseResult<'a, OperatorStmt> {
    Err(ParserError::NotHandledOp(block.opcode))
}
