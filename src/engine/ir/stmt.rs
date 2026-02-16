//! Statement (Stmt) IR types

use super::Expr;

#[derive(Debug, Clone)]
pub enum CloneTarget {
    Myself,
    Target(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum PenColorParam {
    Color,
    Saturation,
    Brightness,
    Transparency,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlStopMode {
    ThisScript,
    All,
    OtherScriptsInTarget,
}

#[derive(Debug, Clone)]
pub enum SayExpr {
    Text(String),
    Numeric(Expr),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    MotionMoveSteps(Expr),
    MotionSetDirection(Expr),
    MotionChangeX(Expr),
    MotionChangeY(Expr),
    MotionSetX(Expr),
    MotionSetY(Expr),
    MotionGoToXY {
        x: Expr,
        y: Expr,
    },
    DataSetVariable {
        variable_index: usize,
        value: Expr,
    },
    DataChangeVariable {
        variable_index: usize,
        delta: Expr,
    },
    DataReplaceListItem {
        list_index: usize,
        index: Expr,
        item: Expr,
    },
    DataAddToList {
        list_index: usize,
        item: Expr,
    },
    DataDeleteListItem {
        list_index: usize,
        index: Expr,
    },
    DataDeleteAllOfList {
        list_index: usize,
    },
    LooksSwitchCostumeTo(Expr),
    LooksSwitchBackdropTo(Expr),
    LooksSetEffectTo {
        effect: Expr,
        value: Expr,
    },
    LooksSetSize(Expr),
    LooksShow,
    LooksSay(SayExpr),
    SoundPlay,
    ControlRepeat {
        times: Expr,
        body: Vec<Stmt>,
    },
    ControlWait {
        duration: Expr,
    },
    ControlWaitUntil {
        condition: Expr,
    },
    ControlForEach {
        variable_index: usize,
        count: Expr,
        body: Vec<Stmt>,
    },
    ControlForever {
        body: Vec<Stmt>,
    },
    ControlRepeatUntil {
        condition: Expr,
        body: Vec<Stmt>,
    },
    ControlWhile {
        condition: Expr,
        body: Vec<Stmt>,
    },
    ControlIf {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    MotionSetRotationStyle,
    DataShowVariable,
    ControlStop {
        mode: ControlStopMode,
    },
    ControlCreateCloneOf {
        target: CloneTarget,
    },
    ControlDeleteThisClone,
    SensingAskAndWait {
        question: Expr,
    },
    MusicSetTempo {
        tempo: Expr,
    },
    SensingResetTimer,
    LooksHide,
    SensingSetDragMode,
    TextToSpeechSpeakAndWait(Expr),
    PenDown,
    PenUp,
    PenClear,
    PenSetSize(Expr),
    PenSetColor(Expr),
    PenStamp,
    PenSetColorParam {
        param: PenColorParam,
        value: Expr,
    },
    EventBroadcast {
        message: Expr,
        wait: bool,
    },
    ProcedureCall {
        procedure_index: usize,
        args: Vec<Expr>,
    },
    ProcedureReturn {
        value: Expr,
    },
}
