use std::collections::HashMap;

use crate::types::StringOrBool;

pub enum GreaterTarget {
    Volume,
    Timer,
}

pub enum HatStmt {
    WhenFlagClicked,
    WhenKeyPressed {
        key: String,
    },
    WhenThisSpriteClicked,
    WhenBacdropSwitchesTo {
        backdrop: String,
    },
    WhenGreaterThan {
        target: GreaterTarget,
    },
    WhenBroadcastReceived {
        target: String,
    },
    ControlStartAsClone,
    ProcedureDefinition {
        default: HashMap<String, StringOrBool>,
    },
}
struct Stmt {}
