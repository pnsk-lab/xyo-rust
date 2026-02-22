use std::collections::HashMap;

use crate::{
    str_enum,
    types::{BlockOpCodes, StringOrBool},
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Thread {
    hat: HatStmt,
    stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum GreaterTarget {
    Volume,
    Timer,
}

#[derive(Debug)]
pub enum HatStmt {
    WhenFlagClicked,
    WhenKeyPressed {
        key: Keys,
    },
    WhenThisSpriteClicked,
    WhenBacdropSwitchesTo {
        backdrop: usize,
    },
    WhenGreaterThan {
        target: GreaterTarget,
        value: Expr,
    },
    WhenBroadcastReceived {
        target: String,
    },
    ControlStartAsClone,
    ProcedureDefinition {
        default: HashMap<String, StringOrBool>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Motion(MotionStmt),
    Looks(LooksStmt),
    Sound(SoundStmt),
    Event(EventStmt),
    Control(ControlStmt),
    Sensing(SensingStmt),
    Operator(OperatorStmt),
    DataStmt(DataStmt),
    Procedures(ProceduresStmt),
    Pen(PenStmt),
}
#[derive(Debug)]
pub enum MotionStmt {}
#[derive(Debug)]
pub enum LooksStmt {}
#[derive(Debug)]
pub enum SoundStmt {}
#[derive(Debug)]
pub enum EventStmt {}
#[derive(Debug)]
pub enum ControlStmt {}
#[derive(Debug)]
pub enum SensingStmt {}
#[derive(Debug)]
pub enum OperatorStmt {}
#[derive(Debug)]
pub enum DataStmt {}
#[derive(Debug)]
pub enum ProceduresStmt {}
#[derive(Debug)]
pub enum PenStmt {}

#[derive(Debug)]
pub enum Expr {
    Motion(MotionExpr),
    Looks(LooksExpr),
    Sound(SoundExpr),
    Event(EventExpr),
    Control(ControlExpr),
    Sensing(SensingExpr),
    Operator(OperatorExpr),
    Data(DataExpr),
    Procedures(ProceduresExpr),
    Pen(PenExpr),
    Literal(Literal),
}
#[derive(Debug)]
pub enum MotionExpr {
    XPosition,
    YPosition,
    Direction,
}
#[derive(Debug)]
pub enum CostumeStatueTarget {
    Name,
    Number,
}
#[derive(Debug)]
pub enum LooksExpr {
    CostumeStatus { target: CostumeStatueTarget },
    BackdropStatus { target: CostumeStatueTarget },
    Size,
}
#[derive(Debug)]
pub enum SoundExpr {
    Volume,
}
#[derive(Debug)]
pub enum EventExpr {}
#[derive(Debug)]
pub enum ControlExpr {}
#[derive(Debug)]
pub enum TimeTarget {
    Year,
    Month,
    Day,
    Date,
    Hour,
    Minute,
    Second,
}
#[derive(Debug)]
pub enum StatusTarget {
    XPosition,
    YPosition,
    Direction,
    CostumeNumber,
    CostumeName,
    Size,
    Volume,
    Variable(String),
}
#[derive(Debug)]
pub enum SensingExpr {
    TouchingObject {
        target: Box<Expr>,
    },
    TouchingColor {
        target: Box<Expr>,
    },
    ColorTouchingColor {
        base: Box<Expr>,
        target: Box<Expr>,
    },
    DistanceBy {
        target: Box<Expr>,
    },
    Answer,
    IsKeyDown {
        target: Box<Expr>,
    },
    IsMouseDown,
    MouseX,
    MouseY,
    Volume,
    NowTime {
        time: TimeTarget,
    },
    SpriteStatus {
        target: Box<Expr>,
        item: StatusTarget,
    },
    Timer,
    Since2000Days,
    Username,
    Online,
}
#[derive(Debug)]
pub enum CalcOp {
    Abs,
    Floor,
    Ceil,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    LogE,
    Log10,
    PowE,
    Pow10,
}
#[derive(Debug)]
pub enum OperatorExpr {
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Sub {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Mul {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Div {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Random {
        from: Box<Expr>,
        to: Box<Expr>,
    },
    LessThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Eq {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    And {
        left: Option<Box<Expr>>,
        right: Option<Box<Expr>>,
    },
    Or {
        left: Option<Box<Expr>>,
        right: Option<Box<Expr>>,
    },
    Not {
        target: Option<Box<Expr>>,
    },
    Join {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Slice {
        target: Box<Expr>,
        idx: Box<Expr>,
    },
    Len {
        target: Box<Expr>,
    },
    IsInclude {
        target: Box<Expr>,
        content: Box<Expr>,
    },
    Mod {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Round {
        target: Box<Expr>,
    },
    Calc {
        target: Box<Expr>,
        op: CalcOp,
    },
}
#[derive(Debug)]
pub enum DataExpr {
    GetItemOf { target: String, idx: Box<Expr> },
    GetItemIndex { target: String, content: Box<Expr> },
    GetLen { target: String },
    IsInclude { target: String, content: Box<Expr> },
}
#[derive(Debug)]
pub enum ProceduresExpr {
    ProcedureArgument,
}
#[derive(Debug)]
pub enum PenExpr {}
#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(String),
    Variable { target: String },
    List { target: String },
    Color { color: String },
    Broadcast { id: String },
}

#[derive(Debug)]
pub enum ParserError<'a> {
    NotHandledOp(BlockOpCodes),
    InvalidValue(&'a str),
    UnknownBlock(String),
}

impl fmt::Display for ParserError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::NotHandledOp(s) => write!(f, "invalid opcode: {s}"),
            ParserError::InvalidValue(s) => write!(f, "invalid value: {s}"),
            ParserError::UnknownBlock(s) => write!(f, "unknown block id: {s}"),
        }
    }
}

impl Error for ParserError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

pub type ParseResult<'a, T> = Result<T, ParserError<'a>>;

str_enum! {
    pub enum Keys {
        A => "a",
        B => "b",
        C => "c",
        D => "d",
        E => "e",
        F => "f",
        G => "g",
        H => "h",
        I => "i",
        J => "j",
        K => "k",
        L => "l",
        M => "m",
        N => "n",
        O => "o",
        P => "p",
        Q => "q",
        R => "r",
        S => "s",
        T => "t",
        U => "u",
        V => "v",
        W => "w",
        X => "x",
        Y => "y",
        Z => "z",
        ZERO => "0",
        ONE => "1",
        TWO => "2",
        THREE => "3",
        FOUR => "4",
        FIVE => "5",
        SIX => "6",
        SEVEN => "7",
        EIGHT => "8",
        NINE => "9",
        SPACE =>"space",
        LEFT => "left arrow",
        UP => "up arrow",
        RIGHT => "right arrow",
        DOWN => "down arrow",
        ANY => "any",
    }
}
