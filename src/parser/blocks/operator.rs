use crate::{
    parser::{
        parser::parse_input,
        types::{CalcOp, OperatorExpr, ParseResult, ParserError},
    },
    types::{Block, BlockOpCodes, Fields, ScratchProject},
};

pub fn parse_operator_expr<'a>(
    project: &'a ScratchProject,
    target_idx: usize,
    block: &'a Block,
) -> ParseResult<'a, OperatorExpr> {
    match block.opcode {
        BlockOpCodes::OperatorAdd => {
            let inputs = block.inputs.as_ref().unwrap();
            let num1_input = inputs.get("NUM1").unwrap();
            let num1 = parse_input(project, target_idx, num1_input).unwrap();
            let num2_input = inputs.get("NUM2").unwrap();
            let num2 = parse_input(project, target_idx, num2_input).unwrap();
            Ok(OperatorExpr::Add {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorSubtract => {
            let inputs = block.inputs.as_ref().unwrap();
            let num1_input = inputs.get("NUM1").unwrap();
            let num1 = parse_input(project, target_idx, num1_input).unwrap();
            let num2_input = inputs.get("NUM2").unwrap();
            let num2 = parse_input(project, target_idx, num2_input).unwrap();
            Ok(OperatorExpr::Sub {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorMultiply => {
            let inputs = block.inputs.as_ref().unwrap();
            let num1_input = inputs.get("NUM1").unwrap();
            let num1 = parse_input(project, target_idx, num1_input).unwrap();
            let num2_input = inputs.get("NUM2").unwrap();
            let num2 = parse_input(project, target_idx, num2_input).unwrap();
            Ok(OperatorExpr::Mul {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorDivide => {
            let inputs = block.inputs.as_ref().unwrap();
            let num1_input = inputs.get("NUM1").unwrap();
            let num1 = parse_input(project, target_idx, num1_input).unwrap();
            let num2_input = inputs.get("NUM2").unwrap();
            let num2 = parse_input(project, target_idx, num2_input).unwrap();
            Ok(OperatorExpr::Div {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorRandom => {
            let inputs = block.inputs.as_ref().unwrap();
            let from_input = inputs.get("FROM").unwrap();
            let from = parse_input(project, target_idx, from_input).unwrap();
            let to_input = inputs.get("TO").unwrap();
            let to = parse_input(project, target_idx, to_input).unwrap();
            Ok(OperatorExpr::Random {
                from: Box::new(from),
                to: Box::new(to),
            })
        }
        BlockOpCodes::OperatorGt => {
            let inputs = block.inputs.as_ref().unwrap();
            let operand1_input = inputs.get("OPERAND1").unwrap();
            let operand1 = parse_input(project, target_idx, operand1_input).unwrap();
            let operand2_input = inputs.get("OPERAND2").unwrap();
            let operand2 = parse_input(project, target_idx, operand2_input).unwrap();
            Ok(OperatorExpr::GreaterThan {
                left: Box::new(operand1),
                right: Box::new(operand2),
            })
        }
        BlockOpCodes::OperatorLt => {
            let inputs = block.inputs.as_ref().unwrap();
            let operand1_input = inputs.get("OPERAND1").unwrap();
            let operand1 = parse_input(project, target_idx, operand1_input).unwrap();
            let operand2 = inputs.get("OPERAND2").unwrap();
            let operand2 = parse_input(project, target_idx, operand2).unwrap();
            Ok(OperatorExpr::LessThan {
                left: Box::new(operand1),
                right: Box::new(operand2),
            })
        }
        BlockOpCodes::OperatorEquals => {
            let inputs = block.inputs.as_ref().unwrap();
            let operand1_input = inputs.get("OPERAND1").unwrap();
            let operand1 = parse_input(project, target_idx, operand1_input).unwrap();
            let operand2_input = inputs.get("OPERAND2").unwrap();
            let operand2 = parse_input(project, target_idx, operand2_input).unwrap();
            Ok(OperatorExpr::Eq {
                left: Box::new(operand1),
                right: Box::new(operand2),
            })
        }
        BlockOpCodes::OperatorAnd => {
            let inputs = block.inputs.as_ref().unwrap();
            let operand1_input = inputs.get("OPERAND1");
            let operand1 = if operand1_input.is_some() {
                Some(Box::new(
                    parse_input(project, target_idx, operand1_input.unwrap()).unwrap(),
                ))
            } else {
                None
            };
            let operand2_input = inputs.get("OPERAND2");
            let operand2 = if operand2_input.is_some() {
                Some(Box::new(
                    parse_input(project, target_idx, operand2_input.unwrap()).unwrap(),
                ))
            } else {
                None
            };
            Ok(OperatorExpr::And {
                left: operand1,
                right: operand2,
            })
        }
        BlockOpCodes::OperatorOr => {
            let inputs = block.inputs.as_ref().unwrap();
            let operand1_input = inputs.get("OPERAND1");
            let operand1 = if operand1_input.is_some() {
                Some(Box::new(
                    parse_input(project, target_idx, operand1_input.unwrap()).unwrap(),
                ))
            } else {
                None
            };
            let operand2_input = inputs.get("OPERAND2");
            let operand2 = if operand2_input.is_some() {
                Some(Box::new(
                    parse_input(project, target_idx, operand2_input.unwrap()).unwrap(),
                ))
            } else {
                None
            };
            Ok(OperatorExpr::Or {
                left: operand1,
                right: operand2,
            })
        }
        BlockOpCodes::OperatorNot => {
            let inputs = block.inputs.as_ref().unwrap();
            let operand_input = inputs.get("OPERAND");
            let operand = if operand_input.is_some() {
                Some(Box::new(
                    parse_input(project, target_idx, operand_input.unwrap()).unwrap(),
                ))
            } else {
                None
            };
            Ok(OperatorExpr::Not { target: operand })
        }
        BlockOpCodes::OperatorJoin => {
            let inputs = block.inputs.as_ref().unwrap();
            let string1_input = inputs.get("STRING1").unwrap();
            let string1 = parse_input(project, target_idx, string1_input).unwrap();
            let string2_input = inputs.get("STRING2").unwrap();
            let string2 = parse_input(project, target_idx, string2_input).unwrap();
            Ok(OperatorExpr::Join {
                left: Box::new(string1),
                right: Box::new(string2),
            })
        }
        BlockOpCodes::OperatorLetterOf => {
            let inputs = block.inputs.as_ref().unwrap();
            let string_input = inputs.get("STRING").unwrap();
            let string = parse_input(project, target_idx, string_input).unwrap();
            let letter_input = inputs.get("LETTER").unwrap();
            let letter = parse_input(project, target_idx, letter_input).unwrap();
            Ok(OperatorExpr::Slice {
                target: Box::new(string),
                idx: Box::new(letter),
            })
        }
        BlockOpCodes::OperatorLength => {
            let inputs = block.inputs.as_ref().unwrap();
            let string_input = inputs.get("STRING").unwrap();
            let string = parse_input(project, target_idx, string_input).unwrap();
            Ok(OperatorExpr::Len {
                target: Box::new(string),
            })
        }
        BlockOpCodes::OperatorContains => {
            let inputs = block.inputs.as_ref().unwrap();
            let string1_input = inputs.get("STRING1").unwrap();
            let string1 = parse_input(project, target_idx, string1_input).unwrap();
            let string2_input = inputs.get("STRING2").unwrap();
            let string2 = parse_input(project, target_idx, string2_input).unwrap();
            Ok(OperatorExpr::IsInclude {
                target: Box::new(string1),
                content: Box::new(string2),
            })
        }
        BlockOpCodes::OperatorMod => {
            let inputs = block.inputs.as_ref().unwrap();
            let num1_input = inputs.get("NUM1").unwrap();
            let num1 = parse_input(project, target_idx, num1_input).unwrap();
            let num2_input = inputs.get("NUM2").unwrap();
            let num2 = parse_input(project, target_idx, num2_input).unwrap();
            Ok(OperatorExpr::Mod {
                left: Box::new(num1),
                right: Box::new(num2),
            })
        }
        BlockOpCodes::OperatorRound => {
            let inputs = block.inputs.as_ref().unwrap();
            let num_input = inputs.get("NUM").unwrap();
            let num = parse_input(project, target_idx, num_input).unwrap();
            Ok(OperatorExpr::Round {
                target: Box::new(num),
            })
        }
        BlockOpCodes::OperatorMathOp => {
            let fields = block.fields.as_ref().unwrap();
            let field = fields.get("OPERATOR").unwrap();
            let operator = match field {
                Fields::V1(v) => &v.0,
                Fields::V2(v) => &v.0,
            };
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
            let inputs = block.inputs.as_ref().unwrap();
            let num_input = inputs.get("NUM").unwrap();
            let num = parse_input(project, target_idx, num_input).unwrap();
            Ok(OperatorExpr::Calc {
                target: Box::new(num),
                op: operator,
            })
        }
        _ => Err(ParserError::NotHandledOp(block.opcode)),
    }
}
