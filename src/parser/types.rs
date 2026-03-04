use std::{collections::HashMap, hash::Hash};

use crate::{
    str_enum,
    types::{BlockOpCodes, RotationStyle, primitive::StringOrNumber},
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Thread {
    pub hat: HatStmt,
    pub stmts: Vec<Stmt>,
    pub target_idx: usize,
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
        key: String,
    },
    WhenThisSpriteClicked,
    WhenStageClicked,
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
        prototype: ProceduresPrototypeStruct,
    },
    WhenTouchingObject {
        object: Expr,
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
    PenStmt(PenStmt),
}
#[derive(Debug)]
pub enum MotionStmt {
    MoveStep { steps: Expr },
    TurnRight { degrees: Expr },
    TurnLeft { degrees: Expr },
    Goto { to: Expr },
    GotoXY { x: Expr, y: Expr },
    GlideTo { secs: Expr, to: Expr },
    GlideToXY { secs: Expr, x: Expr, y: Expr },
    PointInDirection { direction: Expr },
    PointToTowards { towards: Expr },
    ChangeXBy { dx: Expr },
    SetX { x: Expr },
    ChangeYBy { dy: Expr },
    SetY { y: Expr },
    IfOnEdgeBounce,
    SetRotationStyle { style: RotationStyle },
    AlignScene,  // NoOpだから調査サボります。ごめんね
    ScrollRight, // NoOpだから引数いらない
    ScrollUp,    // NoOpだから引数いらない
}
#[derive(Debug)]
pub enum LooksEffects {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}
#[derive(Debug)]
pub enum LooksFrontback {
    Front,
    Back,
}
#[derive(Debug)]
pub enum LooksFowardBackward {
    Forward,
    Backward,
}
#[derive(Debug)]
pub enum LooksStmt {
    SayForSecs {
        message: Expr,
        secs: Expr,
    },
    Say {
        message: Expr,
    },
    ThinkForSecs {
        message: Expr,
        secs: Expr,
    },
    Think {
        message: Expr,
    },
    SwitchCostumeTo {
        costume: Expr,
    },
    NextCostume,
    SwitchBackdropTo {
        backdrop: Expr,
    },
    NextBackdrop,
    ChangeSizeBy {
        change: Expr,
    },
    SetSizeTo {
        size: Expr,
    },
    ChangeEffectBy {
        change: Expr,
        effect: LooksEffects,
    },
    SetEffectTo {
        value: Expr,
        effect: LooksEffects,
    },
    ClearEffects,
    Show,
    Hide,
    GotoFrontback {
        frontback: LooksFrontback,
    },
    GotoForwardBackwardLayers {
        forward_backward: LooksFowardBackward,
    },
    ChangeStretchBy {
        change: Expr,
    },
    SetStretchTo {
        stretch: Expr,
    },
    HideAllSprites,
    SwitchBackdropToAndWait {
        backdrop: Expr,
    },
}
#[derive(Debug)]
pub enum SoundEffect {
    Pitch,
    Pan,
}
#[derive(Debug)]
pub enum SoundStmt {
    PlayUntilDone { sound: Expr },
    Play { sound: Expr },
    StopAllSounds,
    ChangeSoundEffectBy { value: Expr, target: SoundEffect },
    SetSoundEffectTo { value: Expr, target: SoundEffect },
    ClearSoundEffect,
    ChangeVolumeBy { value: Expr },
    SetVolumeTo { value: Expr },
}
#[derive(Debug)]
pub enum EventStmt {
    Broadcast { target: Expr },
    BroadcastAndWait { target: Expr },
}
#[derive(Debug)]
pub enum StopOption {
    All,
    ThisScript,
    OtherScrriptInSprite,
}
#[derive(Debug)]
pub enum ControlStmt {
    Wait {
        duration: Expr,
    },
    Repeat {
        times: Expr,
        substack: Option<Vec<Stmt>>,
    },
    Forever {
        substack: Option<Vec<Stmt>>,
    },
    If {
        condition: Option<Expr>,
        substack: Option<Vec<Stmt>>,
    },
    IfElse {
        condition: Option<Expr>,
        substack: Option<Vec<Stmt>>,
        substack2: Option<Vec<Stmt>>,
    },
    WaitUntil {
        condition: Option<Expr>,
    },
    RepeatUntil {
        condition: Option<Expr>,
        substack: Option<Vec<Stmt>>,
    },
    RepeatWhile {
        condition: Option<Expr>,
        substack: Option<Vec<Stmt>>,
    },
    AllAtOnce {
        substack: Option<Vec<Stmt>>,
    },
    CreateCloneOf {
        clone_option: Expr,
    },
    DeleteThisClone,
    Stop {
        option: StopOption,
    },
    ForEach {
        variable: String,
        value: Expr,
        substack: Option<Vec<Stmt>>,
    },
    IncrCounter,
    ClearCounter,
}

#[derive(Debug)]
pub enum SensingStmt {
    AskAndWait { question: Expr },
    SetDraggable { draggable: bool },
    ResetTimer,
}
#[derive(Debug)]
pub enum OperatorStmt {}
#[derive(Debug)]
pub enum DataStmt {
    SetVariable { value: Expr, variable: String },
    ChangeVariableBy { value: Expr, variable: String },
    ShowVariable { variable: String },
    HideVariable { variable: String },
    AddToList { item: Expr, list: String },
    DeleteOfList { idx: Expr, list: String },
    DeleteAllOfList { list: String },
    InsertAtList { item: Expr, idx: Expr, list: String },
    ReplaceAtList { item: Expr, idx: Expr, list: String },
    ShowList { list: String },
    HideList { list: String },
}
#[derive(Debug)]
pub enum ProceduresStmt {
    ProceduresCall {
        proccode: String,
        inputs: HashMap<String, Expr>,
    },
}
#[derive(Debug)]
pub enum PenStmt {
    PenClear,
    PenStamp,
    PenDown,
    PenUp,
    SetPenColorToColor { color: Expr },
    ChangePenColorParamBy { color_param: Expr, value: Expr },
    SetPenColorParamTo { color_param: Expr, value: Expr },
    ChangePenSizeBy { size: Expr },
    SetPenSizeTo { size: Expr },
    ChangePenHueBy { hue: Expr },
    ChangePenShadeBy { shade: Expr },
    SetPenHueTo { hue: Expr },
    SetPenShadeTo { shade: Expr },
}

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
    XScroll,
    YScroll,
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
pub enum ControlExpr {
    GetCounter,
}
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
    Userid,
    Online,
    IsLoud,
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
    ArgumentReporterBoolean {
        name: String,
    },
    ArgumentReporterStringNumber {
        name: String,
    },
    ProceduresPrototype {
        prototype: ProceduresPrototypeStruct,
    },
}
#[derive(Debug)]
pub struct ProceduresPrototypeStruct {
    pub proccode: String,
    pub arguments: Vec<Argument>,
    pub warp: bool,
}
#[derive(Debug)]
pub struct Argument {
    pub id: String,
    pub default: StringOrNumber,
    pub name: String,
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
    Null,
}

#[derive(Debug)]
pub enum ParserError<'a> {
    NotHandledOp(BlockOpCodes),
    InvalidValue(&'a str),
    UnknownBlock(String),
    InvalidTargetIndex(usize),
    UnexpectedTopLevelPrimitive(String),
    Context {
        context: String,
        source: Box<ParserError<'a>>,
    },
}

impl fmt::Display for ParserError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::NotHandledOp(s) => write!(f, "invalid opcode: {s}"),
            ParserError::InvalidValue(s) => write!(f, "invalid value: {s}"),
            ParserError::UnknownBlock(s) => write!(f, "unknown block id: {s}"),
            ParserError::InvalidTargetIndex(idx) => {
                write!(f, "invalid target index: {idx}")
            }
            ParserError::UnexpectedTopLevelPrimitive(block_id) => {
                write!(f, "unexpected top-level primitive block: {block_id}")
            }
            ParserError::Context { context, source } => write!(f, "{context}: {source}"),
        }
    }
}

impl Error for ParserError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl<'a> ParserError<'a> {
    pub fn context(self, context: impl Into<String>) -> Self {
        ParserError::Context {
            context: context.into(),
            source: Box::new(self),
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
        ENTER => "enter",
        HYPHEN => "-",
        COMMA => ",",
        DOT => "."
    }
}
