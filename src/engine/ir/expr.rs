//! Expression (Expr) IR types

#[derive(Debug, Clone)]
pub enum MathOp {
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
    Ln,
    Log,
    Exp,
    Exp10,
}

#[derive(Debug, Clone, Copy)]
pub enum SensingCurrentMenu {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLiteral(usize),
    MotionXPosition,
    MotionYPosition,
    SensingMouseX,
    SensingMouseY,
    SensingMouseDown,
    LooksCostumeNumber,
    LooksCostumeName,
    SensingOf {
        object: Box<Expr>,
        property: usize,
    },
    SensingCurrent(SensingCurrentMenu),
    Variable(usize),
    ProcedureArg(usize),
    ProcedureCall {
        procedure_index: usize,
        args: Vec<Expr>,
    },
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    Random(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    StringLength(Box<Expr>),
    StringJoin(Box<Expr>, Box<Expr>),
    StringContains(Box<Expr>, Box<Expr>),
    Round(Box<Expr>),
    LetterOf {
        letter: Box<Expr>,
        string: Box<Expr>,
    },
    ListItem {
        list_index: usize,
        index: Box<Expr>,
    },
    ListItemNum {
        list_index: usize,
        item: Box<Expr>,
    },
    ListLength {
        list_index: usize,
    },
    ListContainsItem {
        list_index: usize,
        item: Box<Expr>,
    },
    KeyPressed(Box<Expr>),
    SensingAnswer,
    SensingTimer,
    SensingDaysSince2000,
    SensingTouchingObject(Box<Expr>),
    SensingTouchingColor(Box<Expr>),
    MathOp {
        op: MathOp,
        value: Box<Expr>,
    },
}
